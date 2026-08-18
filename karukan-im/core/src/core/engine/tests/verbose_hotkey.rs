use super::*;

// --- Ctrl+Shift+V verbose hotkey toggle tests ---

#[test]
fn test_ctrl_shift_v_toggles_verbose_by_default() {
    let mut engine = InputMethodEngine::new();
    assert!(!engine.config.verbose);

    // Ctrl+Shift+V → toggle ON
    let result = engine.process_key(&press_ctrl_shift(Keysym::KEY_V_UPPER));
    assert!(result.consumed);
    assert!(engine.config.verbose);

    // Ctrl+Shift+V again → toggle OFF
    let result = engine.process_key(&press_ctrl_shift(Keysym::KEY_V_UPPER));
    assert!(result.consumed);
    assert!(!engine.config.verbose);
}

#[test]
fn test_ctrl_shift_v_lowercase_toggles() {
    let mut engine = InputMethodEngine::new();
    assert!(!engine.config.verbose);

    // Ctrl+Shift+v (lowercase keysym) → toggle ON
    let result = engine.process_key(&press_ctrl_shift(Keysym::KEY_V));
    assert!(result.consumed);
    assert!(engine.config.verbose);
}

#[test]
fn test_verbose_hotkey_false_leaves_the_key_unconsumed() {
    // The whole point of the flag: with it off, the key must reach the
    // application (e.g. a paste shortcut) instead of being swallowed here.
    let mut engine = InputMethodEngine::with_config(EngineConfig {
        verbose_hotkey: false,
        ..EngineConfig::default()
    });

    let result = engine.process_key(&press_ctrl_shift(Keysym::KEY_V_UPPER));
    assert!(
        !result.consumed,
        "with verbose_hotkey off, Ctrl+Shift+V must not be consumed"
    );
    assert!(
        !engine.config.verbose,
        "the key must not have toggled verbose either"
    );
}

#[test]
fn test_verbose_hotkey_false_keeps_the_initial_verbose_setting() {
    // Disabling the hotkey must not affect the `verbose` value itself —
    // only whether Ctrl+Shift+V can flip it at runtime.
    let mut engine = InputMethodEngine::with_config(EngineConfig {
        verbose_hotkey: false,
        verbose: true,
        ..EngineConfig::default()
    });

    let result = engine.process_key(&press_ctrl_shift(Keysym::KEY_V_UPPER));
    assert!(!result.consumed);
    assert!(
        engine.config.verbose,
        "the initial verbose=true must survive an unconsumed toggle attempt"
    );
}

#[test]
fn test_verbose_hotkey_defaults_to_true_so_the_toggle_still_works() {
    let config = EngineConfig::default();
    assert!(
        config.verbose_hotkey,
        "the default must stay upstream's always-on toggle behaviour"
    );
}
