//! This example demonstrates how `climsg` can provide user with readable UI.
use climsg::show;
use climsg::DynScopedVisitor;

// here we only use TextVisitor, so we will use dummy structured message.
const DUMMY: &dyn erased_serde::Serialize = &();

// note: functions should take `&dyn Visitor` or `&dyn DynScopedVisitor`, not a concrete type.
// this allows selecting one good Visitor at startup
fn do_work(vis: &dyn climsg::Visitor) {
    let foo = "foo";
    show!(vis, DUMMY, "obtained foo = {}", foo,);
    let bar = 275;
    show!(vis, DUMMY, "obtained bar = {}", bar,);
}

fn main() {
    let vis = climsg::TextVisitor::new();
    show!(&vis, DUMMY, "hello, {}", "world",);
    do_work(&vis.scoped("work-item-1"));
    do_work(&vis.scoped("work-item-2"));
}
