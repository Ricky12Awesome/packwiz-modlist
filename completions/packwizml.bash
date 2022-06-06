_packwizml() {
    local i cur prev opts cmds
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${i}" in
            "$1")
                cmd="packwizml"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        packwizml)
            opts="-h -V -p -m -M -o -O -F -v -c -s -r -f --help --version --path --cache --mods --output --force --log-level --color-mode --sort-by --reverse --about --json --format"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --path)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -p)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cache)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --mods)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -m)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --log-level)
                    COMPREPLY=($(compgen -W "Off Error Warn Info Debug Trace" -- "${cur}"))
                    return 0
                    ;;
                -v)
                    COMPREPLY=($(compgen -W "Off Error Warn Info Debug Trace" -- "${cur}"))
                    return 0
                    ;;
                --color-mode)
                    COMPREPLY=($(compgen -W "Auto Always Never" -- "${cur}"))
                    return 0
                    ;;
                -c)
                    COMPREPLY=($(compgen -W "Auto Always Never" -- "${cur}"))
                    return 0
                    ;;
                --sort-by)
                    COMPREPLY=($(compgen -W "Name Title Slug Id" -- "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -W "Name Title Slug Id" -- "${cur}"))
                    return 0
                    ;;
                --format)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

complete -F _packwizml -o bashdefault -o default packwizml
