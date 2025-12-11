app [main!] { cli: platform "https://github.com/roc-lang/basic-cli/releases/download/0.20.0/X73hGh05nNTkDHU06FHC0YfFaQB1pimX7gncRcao5mU.tar.br" }

import cli.Stdout
import cli.Arg exposing [Arg]
import "input.txt" as input : Str

main! : List Arg => Result {} _
main! = |_args|

    result =
        input
        |> Str.split_on("\n")
        |> List.drop_if(Str.is_empty)
        |> List.map_try(parse_rotation)

    when result is
        Ok(rotations) ->
            part1_result =
                rotations
                |> List.walk((50, 0), count_position)
                |> |(_, counts)| Num.to_str counts

            part2_result =
                rotations
                |> List.join_map(split_rotation)
                |> List.walk((50, 0), count_position)
                |> |(_, counts)| Num.to_str counts

            Stdout.line!("part1: ${part1_result}. Part2: ${part2_result}")

        Err(_) -> Stdout.line!("Invalid input")

count_position : (I32, I32), I32 -> (I32, I32)
count_position = |(position, acc), rotation|
    new_position = (position + rotation) % 100
    new_acc = if new_position == 0 then acc + 1 else acc
    (new_position, new_acc)

split_rotation : I32 -> List I32
split_rotation = |rotation|
    List.range({ start: At 1, end: At Num.to_u64(Num.abs(rotation)) })
    |> List.map |_|
        if Num.is_negative rotation then -1 else 1

parse_rotation : Str -> Result I32 [InvalidNumStr, InvalidLine]
parse_rotation = |rotation|
    if Str.starts_with(rotation, "L") then
        dir = Str.to_i32(Str.drop_prefix(rotation, "L"))?
        Ok Num.neg(dir)
    else if Str.starts_with(rotation, "R") then
        Ok Str.to_i32(Str.drop_prefix(rotation, "R"))?
    else
        return Err InvalidLine
