using PyCall

if !@isdefined z3
    z3 = pyimport("z3")
end

function solve_machine(line)
    parts = split(line)

    target = parse.(Int, eachsplit(last(parts)[2:end-1], ','))

    buttons = parts[2:end-1]

    vs = [z3.Int("v$i") for i in 1:length(buttons)]

    o = z3.Optimize()

    for v in vs
        o.add(v >= 0)
    end

    ws = PyObject[0 for _ in 1:length(target)]

    for (j, b) in enumerate(buttons)
        for i_str in eachsplit(b[2:end-1], ',')
            i = parse(Int, i_str) + 1

            ws[i] += vs[j]
        end
    end

    for (w, t) in zip(ws, target)
        o.add(w == t)
    end

    s = z3.Int("s")

    o.add(s == sum(vs))

    o.minimize(s)

    o.check()

    o.model().__getitem__(s).py_value()
end

function part2()
    sum(solve_machine, eachline("$(homedir())/aoc-input/2025/day10/input"))
end
