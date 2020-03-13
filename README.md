## async-dict

字典补全命令行工具。

### deoplete 支持

```
Plug 'wsdjeg/async-dict', {'do': 'cargo build'}
call deoplete#custom#var('minigrep', 'minigrep_executable', {
            \ '_':  'minigrep',
            \})
call deoplete#custom#var('minigrep', 'dicts', {
            \ '_':  'path/to/words.txt',
            \})
```



