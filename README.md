# fract

Fract(ulator) - an RPN fraction calculator for makers.

Perform operations on fractional numbers, approximated to the nearest 16th (per standard tape
measure).

## Overview

This calculator uses reverse Polish notation (also known as RPN, or 'postfix' notation).

Type either a *number* or *operation* at the prompt:

* Numbers are saved on the *stack*.
* Operations are performed on the values in the stack.

Enter numbers first, then the operation to perform on them.
Example: Entering '2', then '5', then '/' yields '2/5'.

Numbers can be entered as decimals or fractions.

Examples: '1.05', '3/4', '1+2/5', '1 1/4'.

All numbers, whether decimal or fractional, are converted to reduced, regular fractions.

Operations are:
 
| Operation | Description |
| --------- | ----------- |
| `+`       | Add the last two numbers on the stack. |
| `-`       | Subtract the last number on the stack from the second-to-last one. |
| `*`       | Multiply the last two numbers on the stack. |
| `/`       | Divide the second-to-last number on the stack by the last one. |
| `s`       | Show the current stack. |
| `o`       | Show all available operations. |
| `h`, `?`  | Show this help. |
| `q`       | Quit the program. |

Pressing **Enter** at an empty prompt will duplicate the last value on the stack.

The result of any operation that yields a value will be printed as a reduced, regular fraction.
If the result is a fraction with a denominator > 16, an approximate result is given in 16ths.

Any values used are removed from the stack, and then the result is pushed back onto the stack,
replacing the used values. Unused values remain on the stack.

