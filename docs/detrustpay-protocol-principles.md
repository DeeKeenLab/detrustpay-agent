# DeTrustPay Protocol Principles

- Status: review draft
- Design source: *DeTrustPay Protocol Whitepaper*, version 1.7, May 2026
- Audience: protocol maintainers, SDK authors, agent developers, integrators,
  auditors, and product writers
- Scope: the DeTrust Mechanism, DeTrust Protocol, and agent-facing DeTrustPay
  product interfaces

## Protocol thesis

DeTrustPay is a structured settlement protocol for Promise-based transactions.
It addresses transactions in which payment, delivery, inspection, performance,
or confirmation depends on future counterparty behavior.

The protocol turns an ordinary **Promise** into a **Structured Promise** by
placing it inside:

- defined terms;
- Mutual Economic Exposure;
- recognized actions and response paths;
- deadlines and silence rules; and
- predefined settlement and failure consequences.

The problem is not simply that people lack identity, reputation, honesty, or
communication. The deeper problem is often an asymmetric payoff structure: one
party must become vulnerable while the other party can delay, refuse,
disappear, underperform, or bargain opportunistically at little cost.

DeTrustPay redesigns that structure. Its purpose is not to eliminate trust or
guarantee satisfaction. Its purpose is to reduce mandatory personal trust by
making fair cooperation safer and unfair behavior more costly.

## Mechanism hierarchy

These concepts are related but must not be collapsed:

| Layer | Meaning |
| --- | --- |
| Promise | The external human commitment about future performance, payment, delivery, response, or settlement. |
| Structured Promise | A Promise with defined terms, exposure, response paths, deadlines, and consequences. |
| MEE | Mutual Economic Exposure: both parties accept meaningful locked exposure before the vulnerable stage begins. |
| DDE | The base double-deposit pattern used to implement MEE. |
| eDDE | Enhanced double-deposit logic governing dispute actions, proposals, refusals, silence, deadlines, and terminal outcomes. |
| DeTrust Mechanism | The economic model combining MEE, DDE/eDDE, recognized actions, and predefined consequences. |
| DeTrust Protocol | The state-machine implementation defining states, valid actions, deadlines, locked value, and settlement rules. |
| DeTrustPay | The product layer presenting terms, warnings, timelines, actions, and settlement previews to participants. |

The DeTrust Mechanism is broader than a particular blockchain. The current
Solana program is one implementation of part of the protocol model.

## Core mechanism: Mutual Economic Exposure

For a transaction with payment `P`, payer deposit `Dp`, and payee deposit `Dr`:

```text
Payer locked value = P + Dp
Payee locked value = Dr
Total locked value = P + Dp + Dr
```

The payment is the price owed under normal fulfillment. Deposits are separate
exposure instruments. They alter the payoff of non-cooperation by ensuring
that both parties have economically meaningful value governed by the agreed
transaction rules.

Exposure does not mean automatic forfeiture. Deposits remain subject to the
predefined return, fee, adjustment, cancellation, expiration, and failure
rules of the applicable transaction profile.

Deposit sizing must balance mechanism strength and access. Symbolic deposits
may leave abuse cheap; excessive deposits may exclude honest participants.
Category, ambiguity, reversibility, potential loss, counterparty history, and
lock duration may all affect appropriate exposure.

## Fairness standard

DeTrustPay seeks **fairness of position before settlement**, not guaranteed
satisfaction after settlement.

The payer should not begin from a position in which payment can be taken
without meaningful performance-side exposure. The payee should not begin from
a position in which performance must occur without meaningful payment-side
exposure. During disagreement, neither participant should receive a free
option to exploit refusal, extreme proposals, delay, or silence.

The protocol does not determine moral truth or promise that every dispute will
end in agreement. It structures the positions, actions, exposure, and
consequences under which the participants attempt to settle.

## Chain enforcement and participant responsibility

In a non-custodial on-chain implementation, locked value is controlled by
program rules rather than unilateral custody by either participant or a
service operator.

Before entering locked exposure, both parties are responsible for reviewing
and accepting the complete transaction structure. Once value is locked:

- participant instructions require the signer and role defined by the state;
- a deterministic protocol process may apply a deadline, silence, expiration,
  or terminal rule that was defined before commitment;
- no administrator, marketplace operator, support agent, or other unrelated
  party may invent a new value-affecting rule for that transaction; and
- settlement follows the state machine and accounting rules the participants
  entered.

A predefined timeout or silence consequence is not third-party intervention.
It is execution of a rule accepted before the vulnerable transaction stage
began. Conversely, an off-chain service cannot create such authority merely by
publishing a deadline or deciding that a participant waited too long.

The two parties retain responsibility for the Promise, their counterparties,
the transaction terms, the evidence they exchange, and the actions they sign.
Chain enforcement makes those actions consequential; it does not transfer that
responsibility to DeTrustPay.

## External truth boundary

DeTrustPay controls the settlement layer around an external Promise. It can
control locked value, valid actions, deadlines, state transitions, fees,
exposure, and settlement consequences. It ordinarily cannot directly observe
or judge external performance.

A task manifest, message, receipt, content hash, delivery record, photo, or
other artifact is evidence or a participant assertion. A signature can prove
which address submitted a statement. A content hash can prove that referenced
bytes have not changed. Neither proves that the external Promise was fulfilled
well, truthfully, completely, or acceptably.

In the ordinary MEE/eDDE path, the relevant participant evaluates the Promise
and chooses among the valid response actions. An optional oracle, attestation,
inspection, identity, legal, or platform-support layer may be useful for a
particular category, but it is not the protocol's default source of truth. Any
such layer must be explicitly selected in the transaction template before
funds are locked and must have only the authority that the predefined protocol
profile grants it.

## Recognized actions and eDDE

Base DDE establishes mutual exposure. eDDE extends that structure through a
dispute so disagreement cannot remain an informal, cost-free bargaining game.

Recognized actions may include:

- confirmation;
- refusal;
- proposal and counterproposal;
- acceptance or rejection;
- cancellation;
- expiration;
- silence under an active response rule; and
- terminal failure.

Each recognized dispute action should:

1. record a state transition or action history entry;
2. change or preserve the actions available next; and
3. apply any predefined deadline, fee, exposure, proposal-limit, or
   terminal-risk consequence.

A proposal is a value claim, not a protocol judgment. The counterparty can
accept, reject, or counter according to the applicable profile. Evidence may
support the claim but does not force acceptance unless an explicitly selected
template grants a defined attestation path that effect.

The purpose of eDDE is bounded disagreement: visible actions, visible
deadlines, visible consequences, and a defined route to adjusted settlement or
terminal failure. It does not require agreement at any price.

## Silence, deadlines, and persistence

Silence must never have an improvised meaning. A Structured Promise should
state before commitment whether a missed response means acceptance, rejection,
expiration, escalation, a fee or exposure change, terminal failure, or no
immediate state transition.

The governing principle is:

> Inaction cannot be free leverage, and its consequence cannot be invented
> after funds are locked.

There are two valid implementation conditions:

1. If the on-chain transaction profile contains a silence, deadline, or
   terminal rule, the protocol may apply that predefined consequence without a
   third-party judgment.
2. If the deployed profile contains no such value-affecting rule, the assets
   remain governed by the current on-chain state until a valid participant
   action occurs. A frontend, agent, operator, or off-chain deadline must not
   manufacture a settlement path.

An open transaction under the second condition is not evidence of custody by a
third party or a failure of chain enforcement. It is the result of the state
machine the participants entered.

## Current Solana and Eternal v0 boundary

The whitepaper describes the complete DeTrust Mechanism and target protocol
model. The currently pinned Solana program implements a narrower subset.

For the Eternal v0 agent profile:

- listing acceptance expiration is enforced on-chain;
- proposal versions and optional proposal expiries are enforced on-chain;
- confirmation, provider cancellation, and accepted adjusted settlement are
  recognized program paths;
- confirmation and provider-cancellation fee formulas can change with elapsed
  time;
- proposal-penalty logic exists in the program, but dispute deterrence is
  disabled in the currently verified devnet configuration;
- `deliverBy` is an off-chain task-policy term, not an automatic settlement
  instruction; and
- an accepted order has no general-purpose automatic delivery timeout,
  default-acceptance rule, or terminal-failure distribution path.

Agent software must describe the implemented profile exactly. It must not
market a whitepaper mechanism as deployed before the corresponding state,
instruction, and accounting rules exist on-chain.

## Deterministic state and accounting

Every value-affecting state should define:

- the valid actor or deterministic protocol process;
- valid actions;
- payment, deposit, fee, and exposed value under program control;
- active deadlines and response windows;
- possible next states; and
- the settlement, adjustment, expiration, cancellation, or failure
  consequence.

No value-affecting action should depend on discretionary rules introduced
after funds are locked. Every terminal transition must conserve the transaction
assets according to its predefined accounting rules.

An indexer, explorer, SDK, MCP server, agent, or frontend may interpret and
present on-chain state. It must not become a hidden canonical ledger or gain
settlement authority through convenience.

## Product legibility

Legibility is part of mechanism fairness. Before commitment, DeTrustPay should
make the following understandable:

- who is payer and who is payee;
- what Promise is being backed;
- the payment and each participant's deposit;
- what value is locked;
- the applicable category template;
- the current and possible future states;
- the actions available to each participant;
- the response windows, deadlines, and meaning of silence;
- the fee, exposure, and proposal consequences of each action; and
- how normal, adjusted, cancellation, expiration, and failure settlement would
  distribute value.

A technically valid transaction may still be behaviorally unfair if a product
hides or obscures these consequences.

## Participant responsibilities

Before signing, each participant is responsible for understanding:

- the counterparty and wallet addresses;
- the on-chain program, asset, amount, deposit, role, and vault configuration;
- the external Promise, category template, and acceptance criteria;
- which performance facts remain outside direct protocol knowledge;
- which actions each role or deterministic protocol process may take;
- the response, silence, fee, cancellation, and terminal rules;
- the liquidity cost of locked value; and
- whether the Promise is suitable, lawful, and sufficiently bounded for the
  mechanism.

DeTrustPay cannot make an illegal, coercive, incomprehensible, unbounded, or
fundamentally unsuitable transaction safe merely by adding deposits.

## Agent-facing requirements

Every DeTrustPay agent interface should:

1. Read canonical on-chain state before constructing a write.
2. Verify the cluster, program, mint, participants, PDAs, vaults, amounts,
   version, current state, and signer role.
3. Resolve the Structured Promise and identify any mismatch between its terms
   and enforceable on-chain values.
4. Show the asset, fee, exposure, deadline, and next-state consequences before
   requesting authorization.
5. Use an external wallet or explicit participant policy signer for every
   value-affecting action.
6. Treat a delivery receipt as a participant assertion and integrity reference,
   not verified fulfillment.
7. Never represent model evaluation, an artifact hash, an off-chain timer, or
   operator opinion as a protocol decision.
8. Apply silence or deadline semantics only when they are part of the selected
   enforceable profile; otherwise report them as off-chain policy.
9. Report open transactions by observed state and available actions without
   promising administrator rescue or automatic settlement.
10. Preserve enough action and transaction evidence for participants to audit
    what rule and signature caused each transition.

Agent automation may recommend, construct, simulate, monitor, or notify. It
must not confuse an agent-generated conclusion with participant authorization
or an on-chain observation of external truth.

## Protocol invariants for revisions

Future revisions should preserve these invariants:

- **Structured terms before exposure:** value-affecting terms and consequences
  are defined before both parties enter the vulnerable stage.
- **Mutual Economic Exposure:** both parties carry meaningful exposure rather
  than one side receiving a free exploitation position.
- **Deterministic authority:** only the defined participant or protocol process
  may cause a valid transition.
- **No discretionary surprise:** no administrator or operator invents a new
  settlement rule after commitment.
- **External truth boundary:** the protocol does not claim direct knowledge of
  off-chain fulfillment it cannot observe.
- **Consequential dispute actions:** proposal, refusal, delay, and silence are
  governed by the selected eDDE profile rather than informal leverage.
- **Fairness of position:** the mechanism improves the parties' starting and
  bargaining structure; it does not guarantee satisfaction.
- **Deterministic accounting:** every settlement and failure path conserves and
  distributes controlled value according to predefined rules.
- **Product legibility:** participants can understand exposure and consequences
  before authorizing them.
- **Implementation honesty:** product claims distinguish the complete protocol
  model from the subset deployed in a particular program version.

Program-wide configuration and upgrade authority are governance concerns, not
ordinary transaction-level adjudication. They must be documented separately
and must not be presented as a support operator's power to select the outcome
of an individual order.

## Language guide

Preferred descriptions:

- Structured Promise
- Mutual Economic Exposure (MEE)
- enhanced double-deposit logic (eDDE)
- participant-controlled, chain-enforced settlement
- mechanism-backed fairness
- fairness of position
- recognized transaction action
- predefined silence rule
- adjusted settlement
- terminal failure
- participant-signed assertion or evidence

Descriptions to avoid:

- simple escrow
- trustless or risk-free transaction
- guaranteed fair outcome
- protocol-verified external delivery
- administrator-resolved dispute
- automatic rescue
- off-chain deadline enforced by the chain
- permanently locked funds as a generic protocol weakness

## Review questions

This draft is ready for maintainer revision. Review should confirm:

- the exact mapping between whitepaper v1.7 states and the current Solana
  instructions;
- which eDDE silence, refusal, rejection, proposal-limit, fee-pressure, and
  terminal-failure rules belong in the next on-chain version;
- which category templates are suitable for the agent profile;
- how the product should explain deposit sizing and liquidity exposure;
- how optional evidence, oracle, identity, legal, or compliance layers remain
  visibly separate from the core mechanism; and
- whether this document should become a normative protocol specification or
  remain an integration and product-principles document.
