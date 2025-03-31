```
enum Result[T, E] {
    Ok(T),
    Error(E)
}

define foo(x: int) -> Result[int, string] = {
    let num = 1
    if num is 0 then
        return Error('should not happen')
    otherwise
        print('hello')

    while num is not 0 {
        print(num)
        num = 0
    }

    Ok(x * x)
}

define main() = {
    match foo(2) {
        case Ok(result) -> print('2 * 2 = $result')
        case Error(msg) -> print('error occurred $msg')
    }
}
```