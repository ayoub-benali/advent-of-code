app [main!] { cli: platform "https://github.com/roc-lang/basic-cli/releases/download/0.20.0/X73hGh05nNTkDHU06FHC0YfFaQB1pimX7gncRcao5mU.tar.br" }

import cli.Stdout
import cli.Arg exposing [Arg]
import "input.txt" as input : Str

main! : List Arg => Result {} _
main! = |_args|
    part1Result = process_part(input, invalid_id_part1)?
    part2Result = process_part(input, invalid_id_part2)?

    Stdout.line!("part 1: ${part1Result}\npart 2: ${part2Result}")

process_part = |test_input, is_valid|
    result =
        test_input
        |> Str.trim
        |> parse_string

    Result.map_ok(
        result,
        |str|
            str
            |> process_range is_valid
            |> Num.to_str,
    )

process_range : List (List (Num a)), (Num a -> Bool) -> Num a
process_range = |ranges, is_valid|
    List.join_map(
        ranges,
        |range|
            when range is
                [head, tail] -> find_invalid_values(head, tail, is_valid)
                _ -> [],
    )
    |> List.sum

parse_string : Str -> Result (List (List U64)) [InvalidNumStr]
parse_string = |str|
    str
    |> Str.split_on(",")
    |> List.map_try(
        |id_range|
            Str.split_on(id_range, "-")
            |> List.map_try(|bound| Str.to_u64(bound)),
    )

find_invalid_values : Num a, Num a, (Num a -> Bool) -> List (Num a)
find_invalid_values = |start, end, is_invalid|
    List.range({ start: At start, end: At end })
    |> List.keep_if is_invalid

invalid_id_part1 : Num * -> Bool
invalid_id_part1 = |id|
    s = Num.to_str(id)
    items = Str.to_utf8(s)
    len = List.len(items)

    if Num.is_odd(len) then
        Bool.false
    else
        { before, others } = List.split_at(items, Num.ceiling(Num.to_frac(len) / 2))
        before == others
#
invalid_id_part2 : Num * -> Bool
invalid_id_part2 = |id|
    s = Num.to_str(id)
    items = Str.to_utf8(s)
    len = List.len(items)

    List.range({ start: At 1, end: At Num.ceiling(Num.to_frac(len) / 2) })
    |> List.any |chunk_size|
        chunks = List.chunks_of(items, chunk_size)
        when chunks is
            [_head] -> Bool.false
            [head, .. as tail] -> List.all(tail, |e| e == head)
            _ -> Bool.false

expect
    res = invalid_id_part1(11331133)
    res == Bool.true

expect
    res = invalid_id_part1(11)
    res == Bool.true

expect
    res = invalid_id_part2(11)
    res == Bool.true

expect
    res = invalid_id_part2(113311331133)
    res == Bool.true

expect
    res = invalid_id_part2(113311335)
    res == Bool.false

expect
    test_input = "11-22,95-115,998-1012,1188511880-1188511890"
    res = parse_string(test_input)
    res == Ok [[11, 22], [95, 115], [998, 1012], [1188511880, 1188511890]]
