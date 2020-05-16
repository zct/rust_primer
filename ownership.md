
# OwnerShip

* Each value in Rust has a variable that’s called its owner.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.

引用不会导致ownership的转移;

有refrence就需要life time annotation？ 是的

但是编译器可以通过以下规则推导，如果可以推导出所有字段（参数和返回值）的life time，就不需要指定：

1. The first rule is that each parameter that is a reference gets its own lifetime parameter
2. The second rule is if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters:
3. The third rule is if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters.
   
   


