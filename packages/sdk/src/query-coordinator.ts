export type RpcQueryOptions = {
  force?: boolean;
  retries?: number;
  ttlMs?: number;
};

export type RpcQueryCoordinatorOptions = {
  baseRetryDelayMs?: number;
  defaultRetries?: number;
  defaultTtlMs?: number;
  maxConcurrency?: number;
  now?: () => number;
  random?: () => number;
  sleep?: (milliseconds: number) => Promise<void>;
};

type CacheEntry<T> = {
  expiresAt: number;
  value: T;
};

const defaultSleep = (milliseconds: number) =>
  new Promise<void>((resolve) => setTimeout(resolve, milliseconds));

export function isRpcRateLimitError(error: unknown): boolean {
  const message = error instanceof Error ? error.message : String(error);
  return /\b429\b|too many requests|rate.?limit/i.test(message);
}

/**
 * Coordinates user-driven RPC reads without starting background polling.
 * Identical reads share one promise, concurrency stays bounded, and an RPC 429
 * establishes a coordinator-wide cooldown so unrelated reads do not amplify it.
 */
export class RpcQueryCoordinator {
  readonly #baseRetryDelayMs: number;
  readonly #defaultRetries: number;
  readonly #defaultTtlMs: number;
  readonly #maxConcurrency: number;
  readonly #now: () => number;
  readonly #random: () => number;
  readonly #sleep: (milliseconds: number) => Promise<void>;

  readonly #cache = new Map<string, CacheEntry<unknown>>();
  readonly #inFlight = new Map<string, Promise<unknown>>();
  readonly #waiters: Array<() => void> = [];
  #activeQueries = 0;
  #rateLimitCooldownUntil = 0;

  constructor(options: RpcQueryCoordinatorOptions = {}) {
    this.#baseRetryDelayMs = options.baseRetryDelayMs ?? 750;
    this.#defaultRetries = options.defaultRetries ?? 3;
    this.#defaultTtlMs = options.defaultTtlMs ?? 30_000;
    this.#maxConcurrency = options.maxConcurrency ?? 2;
    this.#now = options.now ?? Date.now;
    this.#random = options.random ?? Math.random;
    this.#sleep = options.sleep ?? defaultSleep;

    if (!Number.isInteger(this.#maxConcurrency) || this.#maxConcurrency < 1) {
      throw new RangeError("maxConcurrency must be a positive integer.");
    }
  }

  async query<T>(
    key: string,
    operation: () => Promise<T>,
    options: RpcQueryOptions = {},
  ): Promise<T> {
    if (!key) {
      throw new TypeError("RPC query key must not be empty.");
    }

    const cached = this.#cache.get(key) as CacheEntry<T> | undefined;
    if (!options.force && cached && cached.expiresAt > this.#now()) {
      return cached.value;
    }

    const existing = this.#inFlight.get(key) as Promise<T> | undefined;
    if (existing) {
      return existing;
    }

    const request = this.#run(key, operation, options);
    this.#inFlight.set(key, request);
    return request;
  }

  invalidate(prefix?: string): void {
    if (prefix === undefined) {
      this.#cache.clear();
      return;
    }
    for (const key of this.#cache.keys()) {
      if (key.startsWith(prefix)) {
        this.#cache.delete(key);
      }
    }
  }

  async #run<T>(
    key: string,
    operation: () => Promise<T>,
    options: RpcQueryOptions,
  ): Promise<T> {
    await this.#acquireSlot();
    try {
      const value = await this.#runWithRetry(
        operation,
        options.retries ?? this.#defaultRetries,
      );
      const ttlMs = options.ttlMs ?? this.#defaultTtlMs;
      if (ttlMs > 0) {
        this.#cache.set(key, { value, expiresAt: this.#now() + ttlMs });
      }
      return value;
    } finally {
      this.#releaseSlot();
      this.#inFlight.delete(key);
    }
  }

  async #runWithRetry<T>(
    operation: () => Promise<T>,
    retries: number,
  ): Promise<T> {
    if (!Number.isInteger(retries) || retries < 0) {
      throw new RangeError("retries must be a non-negative integer.");
    }

    let attempt = 0;
    while (true) {
      await this.#waitForCooldown();
      try {
        return await operation();
      } catch (error) {
        if (!isRpcRateLimitError(error) || attempt >= retries) {
          throw error;
        }
        const jitter = Math.floor(this.#random() * 250);
        const backoff = this.#baseRetryDelayMs * 2 ** attempt + jitter;
        this.#rateLimitCooldownUntil = Math.max(
          this.#rateLimitCooldownUntil,
          this.#now() + backoff,
        );
        attempt += 1;
      }
    }
  }

  async #waitForCooldown(): Promise<void> {
    const remaining = this.#rateLimitCooldownUntil - this.#now();
    if (remaining > 0) {
      await this.#sleep(remaining);
      this.#rateLimitCooldownUntil = this.#now();
    }
  }

  async #acquireSlot(): Promise<void> {
    if (this.#activeQueries < this.#maxConcurrency) {
      this.#activeQueries += 1;
      return;
    }
    await new Promise<void>((resolve) => this.#waiters.push(resolve));
    this.#activeQueries += 1;
  }

  #releaseSlot(): void {
    this.#activeQueries = Math.max(0, this.#activeQueries - 1);
    this.#waiters.shift()?.();
  }
}
