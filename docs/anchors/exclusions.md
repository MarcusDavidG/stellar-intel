# Anchor Exclusions

Domains confirmed as **not eligible** for integration into Stellar Intel's
corridor lists. Each entry records the reason for exclusion so future
contributors do not duplicate the triage work.

Issuer-only / non-fiat anchors that were identified via the directory survey
are also tracked in the fleet recheck obligation (issue **#B065**).

---

## Excluded domains

### `naobtc.com`

| Field           | Value                                    |
| --------------- | ---------------------------------------- |
| Issue           | #467 (B034)                              |
| Triaged         | 2026-06-28                               |
| Category        | Crypto-only — no fiat corridor           |
| SEP-38 `/info`  | No fiat `sell_asset` / `buy_asset` pairs |
| Linked tracking | #B065 (issuer-only / non-fiat register)  |

**Decision:** naobtc.com is a BTC anchor with no fiat off-ramp or on-ramp
corridor. Its `/info` endpoint exposes no fiat asset pairs. It must not be
added to any corridor list and requires no further SEP-24/SEP-38 integration
work. Exclusion is permanent unless the operator adds a verifiable fiat
corridor and re-submits via the anchor onboarding flow described in
[`docs/ANCHOR_ONBOARDING.md`](../ANCHOR_ONBOARDING.md).

### `stellarport.io`

| Field    | Value                                   |
| -------- | --------------------------------------- |
| Issue    | #468                                    |
| Category | DEX / crypto gateway — no fiat corridor |

**Decision:** Stellarport is primarily a Decentralized Exchange (DEX) and
gateway for crypto assets (BTC, ETH, XRP, LTC). Verification of its
`stellar.toml` reveals that all issued assets are crypto-anchored
(`anchor_asset_type="crypto"`). Furthermore, its transfer server endpoint
(`a3s.api.stellarport.io`) is unresponsive/non-existent. There is no evidence
of fiat settlement or fiat off-ramp services, so it is not suitable for fiat
off-ramp integration.
