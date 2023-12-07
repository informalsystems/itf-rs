module SumTypes {

  type Option =
    | Some(int)
    | None

  var value: Option

  action init = all {
    value' = None
  }

  action setValue(x: int): bool = all {
    value' = Some(x)
  }

  action incrValue = all {
    match value {
      | Some(x) => value' = Some(x + 1)
      | None => value' = None
    }
  }

  run exampleTest = init.then(setValue(40)).then(incrValue).then(incrValue)

}
