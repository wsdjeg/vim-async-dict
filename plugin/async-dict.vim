if !exists('g:async_dicts')
    ""
    " Setting the directory file path.
    " >
    "   let g:async_dicts = '~/words/dict.txt'
    " <
    let g:async_dicts = []
endif

if !exists('g:async_minigrep_executable')
    ""
    " Setting the executable of minigrep.
    " >
    "   let g:async_minigrep_executable = 'minigrep'
    " <
    let g:async_minigrep_executable = 'minigrep'
endif
