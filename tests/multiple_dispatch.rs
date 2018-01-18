

trait HasName { fn name() -> &'static str; }

struct A;
impl HasName for A { fn name() -> &'static str { "A" } }
struct B;
impl HasName for B { fn name() -> &'static str { "B" } }

fn get_name<T>(obj: &T) -> &'static str where T: HasName {
	T::name()
}

#[test]
fn static_dispatch_test() {
	assert_eq!(get_name(&A{}), "A");
	assert_eq!(get_name(&B{}), "B");
}
