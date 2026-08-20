const assert = require("node:assert/strict");
const { randomBytes } = require("node:crypto");
const anchor = require("@coral-xyz/anchor");
const {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  createMint,
  getAccount,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} = require("@solana/spl-token");

const {
  BN,
  web3: { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction },
} = anchor;

const PROGRAM_ID = new PublicKey("3S3kY64L8a3torKLpqEzkQeqDX82wjKRFXDMvxq2KcnL");
const UPGRADEABLE_LOADER_ID = new PublicKey(
  "BPFLoaderUpgradeab1e11111111111111111111111",
);
const CONFIG_SEED = Buffer.from("config");
const LISTING_SEED = Buffer.from("listing");
const LISTING_VAULT_SEED = Buffer.from("listing_token_vault");
const PARTY_ORDER_SEED = Buffer.from("party_order");
const BUYER_SEED = Buffer.from("buyer");
const SELLER_SEED = Buffer.from("seller");
const ORDER_AUTHORITY_SEED = Buffer.from("order_authority");
const ORDER_VAULT_SEED = Buffer.from("order_token_vault");
const PROTOCOL_FEE_VAULT_SEED = Buffer.from("protocol_fee_vault");

function pda(seeds) {
  return PublicKey.findProgramAddressSync(seeds, PROGRAM_ID)[0];
}

function u64Le(value) {
  return new BN(value).toArrayLike(Buffer, "le", 8);
}

describe("dual order PDA lifecycle", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = new anchor.Program(
    require("../target/idl/detrustpay.json"),
    provider,
  );
  const connection = provider.connection;
  const buyer = provider.wallet;
  const seller = Keypair.generate();
  const isLocalValidator = connection.rpcEndpoint.includes("127.0.0.1");

  it("creates, queries, synchronizes, settles, and explicitly closes both copies", async () => {
    if (isLocalValidator) {
      const signature = await connection.requestAirdrop(
        seller.publicKey,
        10 * LAMPORTS_PER_SOL,
      );
      await connection.confirmTransaction(signature, "confirmed");
    } else {
      await provider.sendAndConfirm(
        new Transaction().add(
          SystemProgram.transfer({
            fromPubkey: buyer.publicKey,
            toPubkey: seller.publicKey,
            lamports: LAMPORTS_PER_SOL / 10,
          }),
        ),
      );
    }

    const mint = await createMint(
      connection,
      buyer.payer,
      buyer.publicKey,
      null,
      6,
    );
    const buyerToken = await getOrCreateAssociatedTokenAccount(
      connection,
      buyer.payer,
      mint,
      buyer.publicKey,
    );
    const sellerToken = await getOrCreateAssociatedTokenAccount(
      connection,
      buyer.payer,
      mint,
      seller.publicKey,
    );
    await mintTo(
      connection,
      buyer.payer,
      mint,
      buyerToken.address,
      buyer.publicKey,
      10_000_000,
    );
    await mintTo(
      connection,
      buyer.payer,
      mint,
      sellerToken.address,
      buyer.publicKey,
      10_000_000,
    );

    const config = pda([CONFIG_SEED]);
    const programData = PublicKey.findProgramAddressSync(
      [PROGRAM_ID.toBuffer()],
      UPGRADEABLE_LOADER_ID,
    )[0];
    if ((await connection.getAccountInfo(config)) === null) {
      await program.methods
        .initializeConfig(buyer.publicKey, true, true, true)
        .accountsStrict({
          authority: buyer.publicKey,
          program: PROGRAM_ID,
          programData,
          configAccount: config,
          systemProgram: SystemProgram.programId,
        })
        .rpc();
    }

    const listingId = randomBytes(16);
    listingId[6] = (listingId[6] & 0x0f) | 0x70;
    listingId[8] = (listingId[8] & 0x3f) | 0x80;
    const listing = pda([LISTING_SEED, listingId]);
    const listingVault = pda([LISTING_VAULT_SEED, listingId]);
    const paymentAmount = new BN(1_000_000);
    const buyerDeposit = new BN(250_000);
    const sellerDeposit = new BN(250_000);

    await program.methods
      .createListingToken(
        [...listingId],
        "Dual PDA lifecycle",
        "Local validator integration test",
        1,
        Keypair.generate().publicKey,
        true,
        paymentAmount,
        buyerDeposit,
        sellerDeposit,
        new BN(1),
        null,
        true,
        true,
        seller.publicKey,
      )
      .accountsStrict({
        creator: buyer.publicKey,
        listing,
        configAccount: config,
        mintAccount: mint,
        creatorTokenAccount: buyerToken.address,
        listingTokenVaultAccount: listingVault,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    const listingAfterCreate = await program.account.listing.fetch(listing);
    assert.equal(listingAfterCreate.creatorOrderRentReserve.isZero(), false);

    const index = new BN(1);
    const indexBytes = u64Le(index);
    const buyerOrder = pda([
      PARTY_ORDER_SEED,
      listing.toBuffer(),
      indexBytes,
      BUYER_SEED,
    ]);
    const sellerOrder = pda([
      PARTY_ORDER_SEED,
      listing.toBuffer(),
      indexBytes,
      SELLER_SEED,
    ]);
    const orderAuthority = pda([
      ORDER_AUTHORITY_SEED,
      listing.toBuffer(),
      indexBytes,
    ]);
    const orderVault = pda([
      ORDER_VAULT_SEED,
      listing.toBuffer(),
      indexBytes,
    ]);

    const sellerBeforeAccept = await connection.getBalance(seller.publicKey);
    await program.methods
      .acceptListingToken(
        [...listingId],
        "accepted",
        false,
        Keypair.generate().publicKey,
        null,
        "ipfs://dual-order-test",
      )
      .accountsStrict({
        counterparty: seller.publicKey,
        configAccount: config,
        listing,
        buyerOrderAccount: buyerOrder,
        sellerOrderAccount: sellerOrder,
        orderAuthority,
        orderTokenVaultAccount: orderVault,
        mintAccount: mint,
        counterpartyTokenAccount: sellerToken.address,
        listingTokenVaultAccount: listingVault,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([seller])
      .rpc();

    const buyerCopy = await program.account.partyOrder.fetch(buyerOrder);
    const sellerCopy = await program.account.partyOrder.fetch(sellerOrder);
    assert(buyerCopy.authority.equals(buyer.publicKey));
    assert(sellerCopy.authority.equals(seller.publicKey));
    assert(buyerCopy.counterparty.equals(seller.publicKey));
    assert(sellerCopy.counterparty.equals(buyer.publicKey));
    assert.deepEqual(Buffer.from(buyerCopy.stateDigest), Buffer.from(sellerCopy.stateDigest));
    assert.equal(buyerCopy.order.id, sellerCopy.order.id);

    const buyerQuery = await program.account.partyOrder.all([
      { memcmp: { offset: 8, bytes: buyer.publicKey.toBase58() } },
    ]);
    const sellerQuery = await program.account.partyOrder.all([
      { memcmp: { offset: 8, bytes: seller.publicKey.toBase58() } },
    ]);
    assert(buyerQuery.some(({ publicKey }) => publicKey.equals(buyerOrder)));
    assert(buyerQuery.every(({ account }) => account.authority.equals(buyer.publicKey)));
    assert.deepEqual(sellerQuery.map(({ publicKey }) => publicKey.toBase58()), [
      sellerOrder.toBase58(),
    ]);

    const buyerOrderInfo = await connection.getAccountInfo(buyerOrder);
    const orderVaultInfo = await connection.getAccountInfo(orderVault);
    const copyRent = await connection.getMinimumBalanceForRentExemption(
      buyerOrderInfo.data.length,
    );
    const vaultRent = await connection.getMinimumBalanceForRentExemption(
      orderVaultInfo.data.length,
    );
    const sellerAfterAccept = await connection.getBalance(seller.publicKey);
    assert.equal(
      sellerBeforeAccept - sellerAfterAccept,
      copyRent + vaultRent,
      "the accepter must ultimately fund only their own order copy plus the vault",
    );

    const protocolFeeVault = pda([
      PROTOCOL_FEE_VAULT_SEED,
      mint.toBuffer(),
    ]);
    await program.methods
      .payerConfirmTokenOrder(buyerCopy.order.id)
      .accountsStrict({
        payer: buyer.publicKey,
        payee: seller.publicKey,
        buyerOrderAccount: buyerOrder,
        sellerOrderAccount: sellerOrder,
        orderAuthority,
        orderTokenVaultAccount: orderVault,
        payerTokenAccount: buyerToken.address,
        payeeTokenAccount: sellerToken.address,
        protocolFeeVault,
        mintAccount: mint,
        configAccount: config,
        listing,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    const closedBuyerCopy = await program.account.partyOrder.fetch(buyerOrder);
    const closedSellerCopy = await program.account.partyOrder.fetch(sellerOrder);
    assert.equal(closedBuyerCopy.order.closed, true);
    assert.equal(closedSellerCopy.order.closed, true);
    assert.equal((await getAccount(connection, orderVault)).amount, 0n);
    assert.deepEqual(
      Buffer.from(closedBuyerCopy.stateDigest),
      Buffer.from(closedSellerCopy.stateDigest),
    );

    await program.methods
      .closeOrderVault()
      .accountsStrict({
        participant: buyer.publicKey,
        buyerOrderAccount: buyerOrder,
        sellerOrderAccount: sellerOrder,
        orderAuthority,
        orderTokenVaultAccount: orderVault,
        vaultRentRefundRecipient: seller.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();
    assert.equal(await connection.getAccountInfo(orderVault), null);

    await program.methods
      .closeMyOrderCopy()
      .accountsStrict({ authority: buyer.publicKey, orderCopy: buyerOrder })
      .rpc();
    await program.methods
      .closeMyOrderCopy()
      .accountsStrict({ authority: seller.publicKey, orderCopy: sellerOrder })
      .signers([seller])
      .rpc();
    assert.equal(await connection.getAccountInfo(buyerOrder), null);
    assert.equal(await connection.getAccountInfo(sellerOrder), null);
    assert.equal(
      await connection.getBalance(seller.publicKey),
      sellerBeforeAccept,
      "the seller must reclaim the order-copy and order-vault rent they funded",
    );

    await program.methods
      .closeListingVault()
      .accountsStrict({
        creator: buyer.publicKey,
        listing,
        mintAccount: mint,
        creatorTokenAccount: buyerToken.address,
        listingTokenVaultAccount: listingVault,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .rpc();
    await program.methods
      .closeListing()
      .accountsStrict({ creator: buyer.publicKey, listing })
      .rpc();
    assert.equal(await connection.getAccountInfo(listingVault), null);
    assert.equal(await connection.getAccountInfo(listing), null);

    if (!isLocalValidator) {
      const remainingSellerLamports = await connection.getBalance(seller.publicKey);
      const refundTransaction = new Transaction().add(
        SystemProgram.transfer({
          fromPubkey: seller.publicKey,
          toPubkey: buyer.publicKey,
          lamports: remainingSellerLamports,
        }),
      );
      await provider.sendAndConfirm(refundTransaction, [seller]);
    }
  });
});
