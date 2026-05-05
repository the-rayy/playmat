# 🃏 Playmat

A multiplayer turn-based card game engine written in Rust. Runs natively on desktop and in the browser via WASM. Designed to support many card games through a plugin interface — one client, many games.

---

## Feature Progress

### 🔐 Authentication & Accounts

| Done | Feature | Priority | Layer | Description |
|:---:|---|:---:|---|---|
 - [ ] | Nickname-only guest mode | MVP | client + server | Enter a name and connect — no registration, no persistence |
| - [ ] | Email + password registration | Core | client + server | Persist account across sessions with hashed passwords server-side |
| - [ ] | Session tokens with expiry | Core | server | JWT or opaque token issued on login, refreshed automatically |
| - [ ] | Persistent login (remember me) | Core | client | Store refresh token locally so users skip login on return |
| - [ ] | OAuth login (Google / GitHub) | Nice | client + server | Third-party sign-in to reduce registration friction |
| - [ ] | Password reset via email | Nice | client + server | Forgot password flow with a time-limited reset link |
| - [ ] | Account deletion | Nice | client + server | Player can permanently delete their account and all data |

---

### 👤 Player Profile

| Done | Feature | Priority | Layer | Description |
|:---:|---|:---:|---|---|
| - [ ] | Display name shown in lobby and game | MVP | client + server | Other players see your name — must work even in guest mode |
| - [ ] | Avatar (uploaded or generated) | Core | client + server | Image shown next to your name throughout the app |
| - [ ] | Editable display name | Core | client + server | Change your name from within the app after registration |
| - [ ] | Stats overview | Core | client + server | Games played, wins, losses, win rate — per game type |
| - [ ] | Match history | Nice | client + server | Paginated list of past games with opponent, result, and date |
| - [ ] | Public profile page | Nice | client + server | Shareable URL showing another player's stats and history |
| - [ ] | Favourite games | Nice | client | Pin preferred game types to the top of the picker |
| - [ ] | Achievement / badge display | Future | client + server | Unlocked badges shown on profile (e.g. 'first win', '100 games') |

---

### 👥 Social & Friends

| Done | Feature | Priority | Layer | Description |
|:---:|---|:---:|---|---|
| - [ ] | In-game chat | Core | client + server | Text channel shared by players in the same room during a game |
| - [ ] | Friend list | Nice | client + server | Add players by name or ID; maintain a persistent friends list |
| - [ ] | Friend requests with accept / decline | Nice | client + server | Pending request state, notifications on accept |
| - [ ] | Online presence indicator | Nice | client + server | Show whether a friend is online, in lobby, or in a game |
| - [ ] | Invite friend to room | Nice | client + server | Send a direct lobby invite to an online friend |
| - [ ] | Block player | Nice | client + server | Prevent a player from sending invites or joining your rooms |
| - [ ] | Emotes / reactions | Future | client + server | Pre-set reactions players can send during a game |
| - [ ] | Recent opponents list | Future | client + server | Quick list of players you've faced recently |

---

### 🚪 Lobby & Matchmaking

| Done | Feature | Priority | Layer | Description |
|:---:|---|:---:|---|---|
| - [ ] | Create a room | MVP | client + server | Host picks game type, player count, and privacy (public / private) |
| - [ ] | Join by room code | MVP | client + server | Share a short code so anyone can join a specific room |
| - [ ] | Ready-up system | MVP | client + server | All players mark ready before the host can start the game |
| - [ ] | Rematch from end-game screen | Core | client + server | One-click rematch with the same players after a game ends |
| - [ ] | Kick player (host only) | Core | client + server | Host removes a player from the room before start |
| - [ ] | Browse open rooms | Nice | client + server | Public list of waiting rooms, filterable by game type |
| - [ ] | Spectator slots | Nice | client + server | Non-playing observers can watch a game in progress |
| - [ ] | Quick match / auto matchmaking | Future | server | Join a queue; server pairs players automatically |
| - [ ] | Ranked queue with skill rating | Future | server | ELO or MMR per game type; track rating over time |

---

### 🎮 Core Gameplay

| Done | Feature | Priority | Layer | Description |
|:---:|---|:---:|---|---|
| - [ ] | Game plugin interface (trait) | MVP | server | `init`, `apply_action`, `is_terminal`, `current_player` — all games implement this |
| - [ ] | Authoritative server game state | MVP | server | All logic runs server-side; clients receive state snapshots |
| - [ ] | Turn enforcement | MVP | server | Server rejects actions from the wrong player |
| - [ ] | Action validation with reason | MVP | server | Server rejects illegal moves and sends a reason back to the client |
| - [ ] | Hidden information per player | MVP | server | Server sends each player only the state they're allowed to see |
| - [ ] | Concede / forfeit | MVP | client + server | Player can resign mid-game; game resolves immediately |
| - [ ] | End-of-game summary screen | MVP | client | Show result and scores before returning to menu |
| - [ ] | Reconnect to game in progress | Core | client + server | Dropped player can rejoin and resume from current state |
| - [ ] | Game clock / turn timer | Nice | client + server | Optional per-turn countdown; auto-pass or forfeit on timeout |
| - [ ] | Undo request (opponent must approve) | Nice | client + server | Request to take back the last action |
| - [ ] | AI / bot opponent | Future | server | Server-side bot implementing the Game trait to fill empty slots |
| - [ ] | Replay / spectate system | Future | client + server | Record action log; replay or stream to spectators |

---

### 🃏 Card Renderer

| Done | Feature | Priority | Layer | Description |
|:---:|---|:---:|---|---|
| - [ ] | Card struct with art slot | MVP | client | Name, text body, value fields, and an art image region |
| - [ ] | Hand layout | MVP | client | Row or fan of cards in hand, scales with hand size |
| - [ ] | Click-to-select + confirm interaction | MVP | client | Select a card, then confirm or pick a target to play it |
| - [ ] | Opponent hand display | MVP | client | Show card backs for opponent's hand with count |
| - [ ] | Deck and discard pile visuals | MVP | client | Stacked backs for deck, top cards visible on discard pile |
| - [ ] | Card hover / inspect | Core | client | Hover or long-press to see a large readable version of a card |
| - [ ] | Valid target highlight | Core | client | Visual cue showing which cards or zones are legal plays |
| - [ ] | Drag-to-play interaction | Nice | client | Drag a card from hand to the play zone as an alternative to click |
| - [ ] | Card play animation | Nice | client | Smooth slide from hand to table when a card is played |
| - [ ] | Card flip animation | Nice | client | Flip from back to front when a face-down card is revealed |

---

### 📦 Deck Builder

| Done | Feature | Priority | Layer | Description |
|:---:|---|:---:|---|---|
| - [ ] | Set active deck before entering a game | Core | client + server | Choose which saved deck to use when queuing |
| - [ ] | Card collection browser | Core | client | Browse all cards for a game type with search and filters |
| - [ ] | Deck list editor | Core | client | Add / remove cards, see card count, enforce deck size rules |
| - [ ] | Deck validation | Core | client | Highlight rule violations (too many copies, wrong type, etc.) |
| - [ ] | Save and name multiple decks | Core | client + server | Persist multiple named decks per game type per player |
| - [ ] | Import / export deck list | Nice | client | Plain text format (list of card names) for sharing decks externally |
| - [ ] | Card rarity / collection tracking | Future | client + server | Track which cards a player owns if the game has collection mechanics |

---

### 🖥️ UI Shell & Navigation

| Done | Feature | Priority | Layer | Description |
|:---:|---|:---:|---|---|
| - [ ] | Screen router | MVP | client | Central screen enum driving which view renders at any time |
| - [ ] | Main menu layout | MVP | client | Game picker, links to profile, settings, friends — the home screen |
| - [ ] | Game picker / catalogue | MVP | client | Grid of available games with name, player count, short description |
| - [ ] | Loading / connecting states | MVP | client | Spinners and status messages while waiting for server responses |
| - [ ] | Error screens | MVP | client | Friendly messages for connection failure, server error, kicked from room |
| - [ ] | Notification system (toasts) | Core | client | In-app toasts for friend requests, invites, turn reminders |
| - [ ] | Responsive layout | Core | client | UI adapts between narrow (browser) and wide (desktop) viewports |
| - [ ] | Screen transition animations | Nice | client | Fade or slide between screens |

---

### ⚙️ Settings

| Done | Feature | Priority | Layer | Description |
|:---:|---|:---:|---|---|
| - [ ] | Audio volume controls | Core | client | Master, music, and SFX sliders |
| - [ ] | Mute toggle | Core | client | Quick mute without opening full settings |
| - [ ] | Notification preferences | Nice | client | Toggle which events trigger in-app notifications |
| - [ ] | Account settings | Nice | client + server | Change email, password, display name from within the app |
| - [ ] | UI scale / font size | Nice | client | Accessibility option for larger text and controls |
| - [ ] | Keybindings / shortcuts | Future | client | Rebind common actions (confirm, cancel, inspect card) |
| - [ ] | Language / localisation | Future | client | Switch display language if multiple locales are supported |

---

### 🌐 Networking & Infrastructure

| Done | Feature | Priority | Layer | Description |
|:---:|---|:---:|---|---|
| - [x] | WebSocket transport | MVP | client + server | Persistent bidirectional connection for real-time game events |
| - [x] | Shared message protocol (typed enums) | MVP | client + server | All client↔server messages defined as enums in the shared crate |
| - [ ] | Heartbeat / ping | MVP | client + server | Periodic ping to detect silent disconnects |
| - [ ] | Reconnect with exponential backoff | Core | client | Client automatically retries on disconnect |
| - [ ] | Rejoin game in progress | Core | server | Server holds state for a dropped player and replays it on reconnect |
| - [ ] | Latency display | Nice | client | Show current ping in the game UI |
| - [ ] | Server-side game state persistence | Future | server | Game state written to a store so a server restart doesn't lose active games |
| - [ ] | Horizontal server scaling | Future | server | Stateless design + external state store to support multiple server instances |
| - [ ] | Delta state updates | Future | client + server | Send only changed fields instead of full state snapshots |

---

### 🖼️ Assets & Content Pipeline

| Done | Feature | Priority | Layer | Description |
|:---:|---|:---:|---|---|
| - [ ] | Image loading — native | MVP | client | Load textures from disk at startup |
| - [ ] | Image loading — WASM | MVP | client | Fetch assets from CDN or bundle into the WASM binary |
| - [ ] | Card art pipeline convention | Core | client | Agreed filename format, resolution, and colour profile for card art |
| - [ ] | Sound effects | Nice | client | Short audio clips for game events (card play, win, error) |
| - [ ] | Hot-reload in dev builds | Nice | client | Watch asset directory and reload textures without restarting |
| - [ ] | Game content in data files | Nice | client + server | Cards and rules defined in TOML/JSON, not hardcoded in Rust |
| - [ ] | Background music | Future | client | Looping ambient tracks per screen / game type |

---

### 🔧 Engine Internals

| Done | Feature | Priority | Layer | Description |
|:---:|---|:---:|---|---|
| - [x] | egui + wgpu integration | MVP | client | egui render pass on top of wgpu surface, native and WASM |
| - [ ] | Game plugin registry | MVP | server | Server map of game ID → `Box<dyn Game>` factory |
| - [ ] | Server-side game test harness | Core | server | Scripted sequences to regression-test game plugins |
| - [ ] | Client-side automated UI tests | Nice | client | Simulate input events to test screen flows without a real server |
| - [ ] | Custom widget layer (replace egui) | Future | client | Bespoke widgets rendered directly via wgpu — only if egui becomes limiting |

---

## Licence

Private project.
