# Regex Detection at Tokenization
This library implements an algorithm to detect if any give forward slash is the beginning of a regular expression literal or should be considered a single forward slash (originally developed by the [sweet.js team](https://github.com/sweet-js/sweet-core/)). This may seem like a strange thing to need to do but the ecma script spec allows for some crazy things regarding division, for example:

```js
let x = {} / 100;
//x == NaN
let y = function() {} / 100;
//y == NaN
{}/1/g //this is actually a regular expression!
```

While most sane JS programmers wouldn't perform the above, it means that we need to look backwards to know if any forward slash might be a regular expression. Keeping a history of tokens is a bit problematic, depending on how long that history needs to be. In this case we may need to look back an arbitrary number of tokens to get the right answer, keeping all of the tokens around indefinitely is pretty expensive. Even if we were to pair down the data to an un-nested enum that would be 1 bytes per token, the library jquery has a total of `46_863` tokens which would be `~45kb`. Add to the overall size and number of allocations the fact that we would need to scan backwards an unknown distance, touching each index, makes this solution less than ideal. So how can we get to a solution? Well, let's take a look at the [sweet.js "read" algorithm](https://github.com/sweet-js/sweet-core/wiki/design).

Initially reading their "almost-one lookbehind" description can be slightly confusing, [they published a paper](https://users.soe.ucsc.edu/~cormac/papers/dls14a.pdf) that details a method for creating "token-trees", the paper goes into much greater detail about what a "token-tree" is but to give you the short version of how it relates to the linked psuedo-code:

- `{}` and `()` are considered one token but represent the full stream between the open and close
- `tok-#` is referring to these "token-trees" not tokens themselves
  - so in `function(n) {} /`, `tok-2` is `)` and `tok-3` is `function`
- The `isBlock` helper function also requires that any `{}` can access a possible parent `{}`
  - so in `{function() {}}` the function body start needs to be able to see the block start at the very beginning
<style>
h4 {
  margin-bottom: 0;
  padding-bottom: 0;
}
</style>
Now let's rephrase the algorithm in plain english.

When we find a forward slash, the first thing we need to do is look backwards 1 token. If the token 1 before the `/` is a punctuation but not `}` or `)` or a keyword but not `this`, we found a regular expression. `}` and `)` are special cases we will get into next but all other previous tokens would mean it is not a regular expression. Now we have just two cases left, first is `)`. If the token before the `/` is a `)`, we need to jump backwards to the token before the `(` that would be paired with it, if that is `if`, `while`, `for`, or `with`, we found a regex otherwise not. If the token one before the `/` is `}`, we need to determine if the pair of `{` and `}` is a "block" ([see below](#is-a-block)). If the `}` isn't part of a "block", it is not a regular expression, if it is a block we need to check if that block is the body of a function expression ([see below](#is-a-function-expression-body)). If the block is the body of a function expression it is not a regular expression otherwise it is a regular expression.

#### Is a Block
To determine if a pair of curly braces is a block we first look 1 before the `{`, if it is a `(`, `[`, a punctuation or keyword that represents an operation ([see below](#punctuation-or-keyword-represents-operation)), or the keyword `case` it is not a block. If the token 1 before the `{` is the keyword `return` or `yield`, we need to compare the line number of the keyword and the `{`, if they match then it is not a block otherwise it is a block. if the token 1 before the `{` is a `:`, we need to look at the possible parent `{`, if there is a parent we run the same test on that `{`, if that is a block, this `{` is also a block, otherwise it is not a block. If the token 1 before the `{` is anything else, it is a block.  

#### Is a Function Expression Body
if the token 1 before the `{` is `)`, we need to look at the two tokens before the paired `(`, if either of them are the keyword `function`, we need to look 1 token that. If the token one before `function` is `(`, `[`, a punctuation or keyword that represents an operation ([see below](#punctuation-or-keyword-represents-operation)), or the keyword `case` or `return` the block is the body of a function expression, in all other cases it is not.

- if the current token is a `/`, look back one token
- if the previous token is `)`
  - check the token before it's `(`
    - if that is `if`, `while`, `for`, or `with`, we found a regex
    - else, we found a forward slash
- if the previous token is `}`
  - we check if it is a block
    - look 1 before it's `{`
      - if that is `(` or `[` it is not a block
      - if that is `:` we look to the `{`'s parent
        - if no parent, it is a block
        - else if the parent is a block, it is a block
        - else, it is not a block
      - if that is a punctuation or keyword that represents an operation (see below), it is not a block
      - if that is the keyword `return` or `yield`
        - check the line number of the open brace and one token before the open brace
          - if they match, it is not a block
          - else, it is a block
      - if that is the keyword `case`, it is not a block
      - else, it is a block
  - if it is a block
    - we look to the token behind the `{`
      - if that is a `)`
        - we check if the token 1 or 2 before the `(` is the keyword `function`, we need to check if that is an expression
          - if the token before `function` is `(`, `[`, punctuation or keyword that represents an operation (see below), or the keyword `case` or `return`, we found a forward slash
          - else, we found a regex
      - else, we found a regex
  - else, we found a forward slash
- if the previous token is any other punctuation, we found a regex
- if the previous token is a keyword but not `this`, we found a regex
- else, we found a forward slash

### Punctuation or Keyword Represents Operation
> `=`, `+=`, `-=`, `*=`, `/=`, `%=`, `<<=`, `>>=`, `>>>=`, `&=`, `|=`, `^=`, `,`, `+`, `-`, `*`, `/`, `%`, `<<`, `>>`, `>>>`, `&`, `|`, `^`, `&&`, `||`, `?`, `:`, 
`instanceof`, `in`, `===`, `==`, `>=`, `<=`, `<`, `>`, `!=`, `!==`, `++`, `--`, `~`, `!`, `delete`, `void`, `typeof`, `throw`, `new`

With that in mind, let's look at an example:

<div style="padding-top: 5px; background:white;">
    <img src="./assets/look_behind.svg" alt="types of tokens" />
</div>

As you can see, each of the tokens has a type, the key describes how we think about tokens when checking for a regular expression. There are 4 types of token we care about the rest get lumped into `other`, we can refer to this set as `MetaToken`. Because of how the `isBlock` works, we need each of these to know what line it was on, so all of the `MetaToken`s will carry their line number. Looking through the above description of our algorithm, the furthest we need to look backwards from an `{` is 3 tokens, so our scanner should always keep track of the last 3 tokens we have seen.


You may have noticed that one of the variants of `MetaToken` is "special punctuation", this is because we need to treat `(`, `)`, `{`, and `}` in a special way.

Using the same example, this is what special means:
<div style="padding: 5px;background: white;">
    <img alt="special punctuation" src="./assets/special_punct.svg" />
</div>
Every `)` or `}` needs to point to the tokens before it's paired `(` or `{` and every `{` needs to point to a parent `{` if one exists. In addition both the `(` and `{` need to point to the 3 tokens before them, which might look something like this:

<div style="padding: 5px;background: white;">
    <img alt="opens with lookbehind" src="./assets/arc_lookbehind.svg" />
</div>
First we encounter the red `(`, it would need to hold the `things` ident at position 1 and `function` keyword at position 2, position 3 would be empty. Next we would encounter the green open curly brace, this would hold the `)` at 1, red `(` at 2 and `things` at 3. Finally we would encounter the blue `{`, this would hold the green `{` at 1, the `)` at 2 and the red `(` at 3.

This means our scanner needs to keep 3 book keeping lists, the first is the last 3 tokens when scanning the next token. This essentially needs to act like a queue with  a fixed size where the `enqueue` action would `dequeue` when full. The next two are going to be stacks of both opening parentheses and opening curly braces. They are stacks because once we find a close, we don't need that open any more. With these three book keeping constructs we can build our chain of parentheses and curly brace pairs. When we encounter an open paren, we push that into both the last three queue and the parentheses stack. When we find a close paren we link that to our open paren and pop it from the parentheses stack. When we find an open curly brace if the last token we have seen is a close paren, we get the open from it and link that to our open brace, we also need to check if the curly brace stack has anything in it, if it does we need to link the _parent_ open curly to this open curly, with all that done we can push this into both the open curly stack and the last three queue. Now will all of that when we find a close brace we can pop the open curly off it's stack and link it to the close, with the open and close connected we can push the close curly onto the last three queue.

With all the book keeping and linking complete, when we find any `/` we can look back at our last three elements. If it is a closed paren, we can use the link to the open, which is holding the three tokens before, if the first token before is one of our keywords, we know this is the start of a regular expression. If it is a close brace, we first check to see if that is the end of a _block_ by following the link to its open and checking one token before that, it that token is a `:` we recursively check the _parent_ opening curly brace, otherwise we look for our special keywords or punctuation. In the event that it _is_ a block, we look one before the opening curly brace, if that is a close parentheses, we check if that is part of a function signature by following the link to the open parentheses and then looking for a function keyword at 1 and 2 before that, if there is a function keyword there, we look one before it to determine if that is a function expression or declaration. WHEW!

Let's take a look at what the last 3 tokens look like when we reach the `/` on line 3.

```rust

[
  MetaToken::CloseParen(
    MetaToken::OpenParen([
      None,
      MetaToken::Keyword(Keyword::Function),
      MetaToken::Ident
    ])
  ),
  MetaToken::OpenBrace {
    look_behind: [
      MetaToken::Ident,
      MetaToken::OpenParen([
        None,
        MetaToken::Keyword(Keyword::Function),
        MetaToken::Ident
      ]),
      MetaToken::CloseParen(
        MetaToken::OpenParen([
            None, 
            MetaToken::Keyword(Keyword::Function),
            MetaToken::Ident
        ])
      ),
    ],
    parent: None
  },
  MetaToken::OpenBrace {
    look_behind: [
      MetaToken::OpenParen([
        None,
        MetaToken::Keyword(Keyword::Function),
        MetaToken::Ident
      ]),
      MetaToken::CloseParen(
        MetaToken::OpenParen([
            None, 
            MetaToken::Keyword(Keyword::Function),
            MetaToken::Ident
        ])
      ),
      MetaToken::OpenBrace {
        look_behind: [
          MetaToken::Ident,
          MetaToken::OpenParen([
            None,
            MetaToken::Keyword(Keyword::Function),
            MetaToken::Ident
          ]),
          MetaToken::CloseParen(
            MetaToken::OpenParen([
                None, 
                MetaToken::Keyword(Keyword::Function),
                MetaToken::Ident
            ])
          ),
        ],
        parent: None
      }
    ],
    parent: Some(MetaToken::OpenBrace {
      look_behind: [
        MetaToken::Ident,
        MetaToken::OpenParen([
          None,
          MetaToken::Keyword(Keyword::Function),
          MetaToken::Ident
        ]),
        MetaToken::CloseParen(
          MetaToken::OpenParen([
              None, 
              MetaToken::Keyword(Keyword::Function),
              MetaToken::Ident
          ])
        ),
      ],
      parent: None
    })
  }
]
```

We have essentially created an linked list and a big one to boot! This means that each time we move 3 past a `}`, we might have a lot of things to `drop` and by default rust does that in a recursive manner ([which can get expensive](https://rust-unofficial.github.io/too-many-lists/first-drop.html)). If we look at our example from above, there are a total of 9 tokens, and when we reach the end of this block, 8 of them are still hanging around in memory. We could try and use some of Rust's smart pointers to make sure we don't have any clones lying around come drop time but picking apart when things can be `Rc`'d and when they cannot be is a pretty challenging problem. Another solution would be to re-write the drop implementation but that just seems like it would get messy fast. A third option is to try and move our regex tests to slightly earlier in the process.

If we look over the logic tree above, we can gather most of the information we need when we encounter any `(`, is the token before it `if`, `while`, `for` or `with`or is the token 1 or 2 before it the keyword `function` and is that an expression? Those are really the two key pieces of information we need. What if we just attached those two booleans to the `(` instead of always linking back to it? Then when we pop the `(` off the paren stack, we can attach the same two booleans to the `)`. Now when we find an `{` we can see if it is a block, we can also attach the close/open paren flags into our `{`, finally we can copy that information over to the `}` when we pop the open off the curly brace stack. While this means we need to do the computation eagerly, it also means we don't have as much to clean up when we move past a `}`. 

With these changes, the last three tokens when we reach the `/` on line 3 would look like this:

```rust
[
  MetaToken::CloseParen {
    is_conditional: false,
    is_func_expr: false,
  },
  MetaToken::OpenBrace {
    is_block: true,
    paren_is_conditional: false,
    paren_is_func_expr: false,
  },
  MetaToken::OpneBrace {
    is_block: true,
    paren_is_conditional: false,
    paren_is_func_expr: false,
  }
]
```
That is much easier to follow, keeps a lot less information around, and solves our possible recursive `drop` problem. We still need to keep around our 3 book keeping lists, though they will be a list of copy types!