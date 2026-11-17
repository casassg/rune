use rune::{Cx, El, Env, Runtime};

use crate::{Script, Sources};

fn view(s: &Script, cx: &mut Cx<Script>) -> El {
    s.view(cx)
}

fn runtime(source: &str) -> Runtime<Script> {
    let mut rt = Runtime::new(Script::new("props", Sources::single(source)), view, (400.0, 400.0), Env::new(true, 1.0));
    rt.frame(0.0);
    rt
}

fn height(rt: &Runtime<Script>, key: &str) -> f32 {
    rt.presented(key).unwrap_or_else(|| panic!("no {key}")).0[3]
}

#[test]
fn max_lines_clamps_text_and_zero_shows_every_line() {
    let rt = runtime(
        r#"local ui = require('@rune/ui')
local words = "one two three four five six seven eight nine ten eleven twelve"
return { render = function()
    return ui.col({}, {
        ui.text(words, { key = "full", w = 80, wrap = true }),
        ui.text(words, { key = "two", w = 80, maxLines = 2 }),
        ui.text(words, { key = "zero", w = 80, maxLines = 0 }),
    })
end }"#,
    );
    let (full, two, zero) = (height(&rt, "full"), height(&rt, "two"), height(&rt, "zero"));
    assert!(two > 0.0 && two < full, "two lines {two} vs all {full}");
    assert_eq!(zero, full);
}

#[test]
fn max_lines_rejects_a_negative_or_fractional_count() {
    for bad in ["-1", "1.5"] {
        let script = Script::new("props", Sources::single(format!("local ui = require('@rune/ui')\nreturn {{ render = function() return ui.text('x', {{ maxLines = {bad} }}) end }}")));
        script.render_uncached::<()>("render", true);
        assert!(script.log().iter().any(|l| l.message.contains("maxLines must be a whole number")), "{bad}: {:?}", script.log());
    }
}

#[test]
fn matched_flies_an_element_across_parents_and_an_unmatched_one_enters_in_place() {
    let mut rt = runtime(
        r#"local ui = require('@rune/ui')
local right = false
local function slot(here: boolean, key: string, id: string?)
    return ui.row({ w = 100, h = 40 }, here and { ui.row({ key = key, matched = id, w = 100, h = 40 }, {}) } or {})
end
return {
    flip = function() right = not right end,
    render = function()
        return ui.col({ w = 400, gap = 20 }, {
            ui.row({ w = 400, justify = "between" }, { slot(not right, "m1", "card"), slot(right, "m2", "card") }),
            ui.row({ w = 400, justify = "between" }, { slot(not right, "p1"), slot(right, "p2") }),
        })
    end,
}"#,
    );
    let x = |rt: &Runtime<Script>, key: &str| rt.presented(key).unwrap().0[0];
    assert_eq!(x(&rt, "m1"), 0.0);
    rt.state.call("flip", []).unwrap();
    rt.invalidate();
    rt.frame(0.0);
    rt.frame(0.03);
    assert_eq!(x(&rt, "p2"), 300.0);
    assert!(x(&rt, "m2") < 299.0, "matched card at {}", x(&rt, "m2"));
}
