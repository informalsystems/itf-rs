module SumTypes {

  type Option =    
    | None
    | Some(int)
    | SomeMore({x: int, y: bool})

  var value: Option

  action init = all {
    value' = None
  }

  action setValue(x: int): bool = all {
    value' = Some(x)
  }

  action addComponent(y: bool) : bool = all {
    match value {
      | Some(x) => value' = SomeMore({x: x, y: y})
      | SomeMore(r) => value' = SomeMore({x: r.x, y: y})
      | None => value' = None
    }
  }

  action incrValue = all {
    match value {
      | Some(x) => value' = Some(x + 1)
      | None => value' = None
      | SomeMore(r) => value' = SomeMore({x: r.x + 1, y: r.y})
    }
  }

  run exampleTest = 
    init
    .then(setValue(40))
    .then(incrValue)
    .then(addComponent(true))
    .then(incrValue)

}
