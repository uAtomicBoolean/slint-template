# slint-template
A Slint template for Rust that implements an Elm-like architecture in the backend.

## Architecture
The backend follows a simple loop: **UI → Message → `update` → State → `view` → UI**.

Everything is built around the `ElmModel` trait (`src/app/elm_model.rs`):
- `update(msg)`: applies a `Message` to the state. This is where the logic lives. It never touches the UI and returns an `Outcome` (`Ok(())` or `Err(Rejected)`).
- `view(ui)`: pushes the current state into the UI.
- `bind(ui, dispatch)`: called once to connect the UI callbacks to `dispatch`.

On the UI side, each feature exposes two Slint globals:
- `FeatureState`: `in` properties written by `view`.
- `FeatureIntents`: callbacks triggered by the UI and turned into messages by `bind`.

`App` (`src/app/mod.rs`) is the root `ElmModel`. It owns the state of every feature, wraps their messages in its own `Message` enum and forwards `update`, `view` and `bind` to them. Every dispatched message triggers `update` then `view`.

## Adding a feature
1. **UI**: declare `FeatureState` and `FeatureIntents` globals in the `.slint` file and export them.
2. **Feature module** (e.g. `src/feature.rs`):
   - a `Message` enum and its `Into<app::Message>` implementation;
   - a `State` struct implementing `ElmModel`.
3. **`App`** (`src/app/mod.rs`):
   - add a `Message::Feature(feature::Message)` variant;
   - add a `feature: feature::State` field;
   - forward the calls in `update`, `view` and `bind`.

See `src/counter.rs` for a complete example.

> [!NOTE]
> `view` runs after every message, so with heavy data it can be worth updating the UI only when needed.
> For example, with a large struct, add a `struct_dirty` flag to the Rust state that is set to `true` only when the struct is modified, and only push the struct to the UI in `view` when that flag is set.
> For a very long list, keep a reference to the Slint model (e.g. an `Rc<VecModel<T>>`) in the state and modify it in place, or implement a custom Slint `Model`.

## Small apps
Features are optional. For a very small app, you can put the state, messages and logic directly in `App`: it implements `ElmModel` exactly like a feature does.

> [!NOTE]
> The current example (`Counter`) is kept simple on purpose, but it is not really what this architecture is meant for. A plain counter could live entirely in Slint. The data stored in a `State` should be either **persistent** (kept across restarts) or **complex** (e.g. a list of structs or a custom Slint `Model`).

## Multi-threading
Slint UI objects can only be used from the UI thread. To send a message from another thread (e.g. after a long task), use `App::send`:

```rust
std::thread::spawn(|| {
    // ... heavy work ...
    let _ = app::App::send(counter::Message::Increment);
});
```

`App::send` schedules the message on the Slint event loop, where it goes through the regular `update` → `view` cycle. Since it runs asynchronously, the `Outcome` returned by `update` is lost.

If your app doesn't use multi-threading, every block marked with `// MULTI-THREADING` can be removed.
