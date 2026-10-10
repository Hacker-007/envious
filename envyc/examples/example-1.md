```
enum Result[T, E] {
    Ok(T),
    Error(E)
}

define foo(x: int) -> Result[int, string] = {
    let num = 1
    if num is 0 then
        return Result.Error('should not happen')
    otherwise
        print('hello')

    while num is not 0 {
        print(num)
        num = 0
    }

    return Result.Ok(x * x)
}

define main() = {
    match foo(2) {
        case Result.Ok(result) -> print('2 * 2 = $result')
        case Result.Error(msg) -> print('error occurred $msg')
    }
}
```