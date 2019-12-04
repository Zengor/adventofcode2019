from enum import Enum
import itertools
import operator

class Instruction(Enum):
    ADD = 1
    MUL = 2
    HALT = 99
    def op(self):
        if self == Instruction.ADD:
            return operator.add
        elif self == Instruction.MUL:
            return operator.mul
        return None

def run_program(codes):
    cursor = 0
    instruction = Instruction(codes[cursor])
    while instruction != Instruction.HALT:
        operation = instruction.op()
        op1, op2, result_pos = (codes[cursor+1], codes[cursor+2], codes[cursor+3])
        codes[result_pos] = operation(codes[op1],codes[op2])
        cursor += 4
        instruction = Instruction(codes[cursor])
    return codes[0]
        
def program_with_inputs(noun, verb, codes):
    codes = list(codes)
    codes[1] = noun
    codes[2] = verb
    return run_program(codes)

def find_desired_output(desired_output, program_str):
    for (noun,verb) in itertools.product(range(100), range(100)):
        if desired_output == program_with_inputs(noun,verb,program_str):
            return 100 * noun + verb
    return None

with open("../input/02-1.txt") as f:
    program = list(map(int,f.read().strip().split(',')))
    
answer1 = program_with_inputs(12, 2, program)
answer2 = find_desired_output(19690720, program)
print("part 1 {} part 2 {}".format(answer1, answer2))
