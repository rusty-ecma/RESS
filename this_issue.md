
# Unicode Escapes are not evaluated for ident_start or ident_continue

`\u2E2F` should error if found at any point in the `ident` path.


For example

```js
let y\u0053x = 2;
let y\u{53}x = 2;
let y^x = 2;
```

The above two lines are technically equivalent, the tokenization step will fail on line 2 but succeed on line 1, but it should not! We are already resolving each of the escaped characters into their char to test they haven't gone outisde of the maximum unicode escape value. We also need to test for these two situation but only when dealing with identifiers (regex also allows for unicode escapes)

In the tokenizer, we are calculating the u32 that the character is represented by in `escaped_with_code_point`.

This could be corrected in a couple of ways, one would be to implement a wrapper around the Unicode module’s two methods that operate on u32 converting the input to a char. Another would be to convert the value in the current method and pass it to the Unicode module function. The third would be to start on #60 and create a cli tool that will generate the Unicode module but with both char and u32 variants of `id_start` and `id_continue`