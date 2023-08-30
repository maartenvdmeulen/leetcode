function isValid(s: string): boolean {
    const stack: string[] = [];
    const characters = s.split("");

    for (let index = 0; index < characters.length; index++) {
        const c = characters[index];
        switch (c) {
            case "(":
            case "[":
            case "{":
                stack.push(c);
                break;
            case ")":
                if ("(" !== stack.pop()) return false;
                break;
            case "]":
                if ("[" !== stack.pop()) return false;
                break;
            case "}":
                if ("{" !== stack.pop()) return false;
                break;
            default:
                throw new Error(`${c} is not valid character`);
        }
    }

    if (stack.length > 0) return false;

    return true;
}

import { assertEquals } from "https://deno.land/std@0.200.0/assert/mod.ts";
{
    Deno.test("test_case_1", () => {
        assertEquals(isValid("()"), true);
    });

    Deno.test("test_case_2", () => {
        assertEquals(isValid("()[]{}"), true);
    });

    Deno.test("test_case_3", () => {
        assertEquals(isValid("(]"), false);
    });
}
