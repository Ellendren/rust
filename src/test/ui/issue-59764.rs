// aux-build:issue-59764.rs
// compile-flags:--extern issue_59764
// edition:2018
// run-rustfix

use issue_59764::foo::makro;
//~^ ERROR unresolved import `issue_59764::foo::makro` [E0432]

makro!(bar);
//~^ ERROR cannot determine resolution for the macro `makro`

fn main() {
    bar();
    //~^ ERROR cannot find function `bar` in this scope [E0425]
}
