if ! exists('s:jobid')
  let s:jobid = 0
endif

let s:scriptdir = resolve(expand('<sfile>:p:h') . '/..')
let s:bin = s:scriptdir . '/target/release/neofib'

function! neofib#nth(n)
  if s:EnsureJob() == 0
    return rpcrequest(s:jobid, 'nth', a:n)
  else
    echoerr "neofib: couldn't call out to job " . s:jobid
  endif
endfunction

function! s:EnsureJob()
  if s:jobid == 0
    let result = s:StartJob()

    if result == -1
      echoerr "neofib: rpc process is not executable"
      return 1
    else
      let s:jobid = result
      call s:ConfigureJob(result)
      return 0
    endif
  else
    return 0
  endif
endfunction

function! s:ConfigureJob(jobid)
  augroup neofib
    autocmd!
    autocmd VimLeavePre * :call s:StopJob()
  augroup END
endfunction

function! s:OnStderr(id, data, event) dict
  echom 'neofib: stderr: ' . join(a:data, "\n")
endfunction

function! s:StartJob()
  if s:jobid == 0
    let id = jobstart([s:bin], { 'rpc': v:true, 'on_stderr': function('s:OnStderr') })
    return id
  else
    return 0
  endif
endfunction

function! s:StopJob()
  if 0 < s:jobid
    augroup neofib
      autocmd!
    augroup END

    call rpcnotify(s:jobid, 'quit')
    let result = jobwait(s:jobid, 500)

    if result == -1
      call jobstop(s:jobid)
    endif

    let s:jobid = 0
  endif
endfunction
