complete -c packwizml -s p -l path -d 'Path to the packwiz directory containing \'pack.toml\'' -r -f -a "(__fish_complete_directories)"
complete -c packwizml -l cache -d 'Set the cache file' -r
complete -c packwizml -s m -l mods -d 'Path to the directory contains all the mod metadata files' -r -f -a "(__fish_complete_directories)"
complete -c packwizml -s o -l output -d 'Set an output file' -r
complete -c packwizml -s v -l log-level -d 'Sets the verbosity of logging' -r -f -a "{Off	,Error	,Warn	,Info	,Debug	,Trace	}"
complete -c packwizml -s c -l color-mode -d 'Sets the color mode' -r -f -a "{Auto	,Always	,Never	}"
complete -c packwizml -s s -l sort-by -d 'Sets the sorting mode' -r -f -a "{Name	,Title	,Slug	,Id	,None	}"
complete -c packwizml -s f -l format -d 'Set a custom format' -r
complete -c packwizml -s h -l help -d 'Print help information'
complete -c packwizml -s V -l version -d 'Print version information'
complete -c packwizml -s M -d 'Disable \'--mods\' being relative to \'--path\''
complete -c packwizml -s O -d 'Disable\'`--output\' being relative to \'--path\''
complete -c packwizml -s F -l force -d 'Overwrites output if it already exists'
complete -c packwizml -s r -l reverse -d 'Sets if sorting should be reverse'
complete -c packwizml -l about -d 'Prints about this program'
complete -c packwizml -l json -d 'Prints json output'
