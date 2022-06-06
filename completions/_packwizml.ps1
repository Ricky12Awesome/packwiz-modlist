
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'packwizml' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'packwizml'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'packwizml' {
            [CompletionResult]::new('-p', 'p', [CompletionResultType]::ParameterName, 'Path to the packwiz directory containing ''pack.toml''')
            [CompletionResult]::new('--path', 'path', [CompletionResultType]::ParameterName, 'Path to the packwiz directory containing ''pack.toml''')
            [CompletionResult]::new('--cache', 'cache', [CompletionResultType]::ParameterName, 'Set the cache file')
            [CompletionResult]::new('-m', 'm', [CompletionResultType]::ParameterName, 'Path to the directory contains all the mod metadata files')
            [CompletionResult]::new('--mods', 'mods', [CompletionResultType]::ParameterName, 'Path to the directory contains all the mod metadata files')
            [CompletionResult]::new('-o', 'o', [CompletionResultType]::ParameterName, 'Set an output file')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'Set an output file')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'Sets the verbosity of logging')
            [CompletionResult]::new('--log-level', 'log-level', [CompletionResultType]::ParameterName, 'Sets the verbosity of logging')
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'Sets the color mode')
            [CompletionResult]::new('--color-mode', 'color-mode', [CompletionResultType]::ParameterName, 'Sets the color mode')
            [CompletionResult]::new('-s', 's', [CompletionResultType]::ParameterName, 'Sets the sorting mode')
            [CompletionResult]::new('--sort-by', 'sort-by', [CompletionResultType]::ParameterName, 'Sets the sorting mode')
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'Set a custom format')
            [CompletionResult]::new('--format', 'format', [CompletionResultType]::ParameterName, 'Set a custom format')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-M', 'M', [CompletionResultType]::ParameterName, 'Disable ''--mods'' being relative to ''--path''')
            [CompletionResult]::new('-O', 'O', [CompletionResultType]::ParameterName, 'Disable''`--output'' being relative to ''--path''')
            [CompletionResult]::new('-F', 'F', [CompletionResultType]::ParameterName, 'Overwrites output if it already exists')
            [CompletionResult]::new('--force', 'force', [CompletionResultType]::ParameterName, 'Overwrites output if it already exists')
            [CompletionResult]::new('-r', 'r', [CompletionResultType]::ParameterName, 'Sets if sorting should be reverse')
            [CompletionResult]::new('--reverse', 'reverse', [CompletionResultType]::ParameterName, 'Sets if sorting should be reverse')
            [CompletionResult]::new('--about', 'about', [CompletionResultType]::ParameterName, 'Prints about this program')
            [CompletionResult]::new('--json', 'json', [CompletionResultType]::ParameterName, 'Prints json output')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
