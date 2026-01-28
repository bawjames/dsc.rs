# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_dsc_global_optspecs
	string join \n c/config= h/help
end

function __fish_dsc_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_dsc_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_dsc_using_subcommand
	set -l cmd (__fish_dsc_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c dsc -n "__fish_dsc_needs_command" -s c -l config -r -F
complete -c dsc -n "__fish_dsc_needs_command" -s h -l help -d 'Print help'
complete -c dsc -n "__fish_dsc_needs_command" -f -a "list"
complete -c dsc -n "__fish_dsc_needs_command" -f -a "add"
complete -c dsc -n "__fish_dsc_needs_command" -f -a "import"
complete -c dsc -n "__fish_dsc_needs_command" -f -a "update"
complete -c dsc -n "__fish_dsc_needs_command" -f -a "emoji"
complete -c dsc -n "__fish_dsc_needs_command" -f -a "topic"
complete -c dsc -n "__fish_dsc_needs_command" -f -a "category"
complete -c dsc -n "__fish_dsc_needs_command" -f -a "group"
complete -c dsc -n "__fish_dsc_needs_command" -f -a "backup"
complete -c dsc -n "__fish_dsc_needs_command" -f -a "completions"
complete -c dsc -n "__fish_dsc_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c dsc -n "__fish_dsc_using_subcommand list" -s f -l format -r -f -a "plaintext\t''
markdown\t''
markdown-table\t''
json\t''
yaml\t''
csv\t''"
complete -c dsc -n "__fish_dsc_using_subcommand list" -s h -l help -d 'Print help'
complete -c dsc -n "__fish_dsc_using_subcommand add" -s i -l interactive
complete -c dsc -n "__fish_dsc_using_subcommand add" -s h -l help -d 'Print help'
complete -c dsc -n "__fish_dsc_using_subcommand import" -s h -l help -d 'Print help'
complete -c dsc -n "__fish_dsc_using_subcommand update" -s m -l max -r
complete -c dsc -n "__fish_dsc_using_subcommand update" -s C -l concurrent
complete -c dsc -n "__fish_dsc_using_subcommand update" -s p -l post-changelog
complete -c dsc -n "__fish_dsc_using_subcommand update" -s h -l help -d 'Print help'
complete -c dsc -n "__fish_dsc_using_subcommand emoji; and not __fish_seen_subcommand_from add help" -s h -l help -d 'Print help'
complete -c dsc -n "__fish_dsc_using_subcommand emoji; and not __fish_seen_subcommand_from add help" -f -a "add"
complete -c dsc -n "__fish_dsc_using_subcommand emoji; and not __fish_seen_subcommand_from add help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c dsc -n "__fish_dsc_using_subcommand emoji; and __fish_seen_subcommand_from add" -s d -l discourse -r
complete -c dsc -n "__fish_dsc_using_subcommand emoji; and __fish_seen_subcommand_from add" -s h -l help -d 'Print help'
complete -c dsc -n "__fish_dsc_using_subcommand emoji; and __fish_seen_subcommand_from help" -f -a "add"
complete -c dsc -n "__fish_dsc_using_subcommand emoji; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c dsc -n "__fish_dsc_using_subcommand topic; and not __fish_seen_subcommand_from pull push sync help" -s h -l help -d 'Print help'
complete -c dsc -n "__fish_dsc_using_subcommand topic; and not __fish_seen_subcommand_from pull push sync help" -f -a "pull"
complete -c dsc -n "__fish_dsc_using_subcommand topic; and not __fish_seen_subcommand_from pull push sync help" -f -a "push"
complete -c dsc -n "__fish_dsc_using_subcommand topic; and not __fish_seen_subcommand_from pull push sync help" -f -a "sync"
complete -c dsc -n "__fish_dsc_using_subcommand topic; and not __fish_seen_subcommand_from pull push sync help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c dsc -n "__fish_dsc_using_subcommand topic; and __fish_seen_subcommand_from pull" -s d -l discourse -r
complete -c dsc -n "__fish_dsc_using_subcommand topic; and __fish_seen_subcommand_from pull" -s h -l help -d 'Print help'
complete -c dsc -n "__fish_dsc_using_subcommand topic; and __fish_seen_subcommand_from push" -s d -l discourse -r
complete -c dsc -n "__fish_dsc_using_subcommand topic; and __fish_seen_subcommand_from push" -s h -l help -d 'Print help'
complete -c dsc -n "__fish_dsc_using_subcommand topic; and __fish_seen_subcommand_from sync" -s d -l discourse -r
complete -c dsc -n "__fish_dsc_using_subcommand topic; and __fish_seen_subcommand_from sync" -s y -l yes
complete -c dsc -n "__fish_dsc_using_subcommand topic; and __fish_seen_subcommand_from sync" -s h -l help -d 'Print help'
complete -c dsc -n "__fish_dsc_using_subcommand topic; and __fish_seen_subcommand_from help" -f -a "pull"
complete -c dsc -n "__fish_dsc_using_subcommand topic; and __fish_seen_subcommand_from help" -f -a "push"
complete -c dsc -n "__fish_dsc_using_subcommand topic; and __fish_seen_subcommand_from help" -f -a "sync"
complete -c dsc -n "__fish_dsc_using_subcommand topic; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c dsc -n "__fish_dsc_using_subcommand category; and not __fish_seen_subcommand_from list copy pull push help" -s h -l help -d 'Print help'
complete -c dsc -n "__fish_dsc_using_subcommand category; and not __fish_seen_subcommand_from list copy pull push help" -f -a "list"
complete -c dsc -n "__fish_dsc_using_subcommand category; and not __fish_seen_subcommand_from list copy pull push help" -f -a "copy"
complete -c dsc -n "__fish_dsc_using_subcommand category; and not __fish_seen_subcommand_from list copy pull push help" -f -a "pull"
complete -c dsc -n "__fish_dsc_using_subcommand category; and not __fish_seen_subcommand_from list copy pull push help" -f -a "push"
complete -c dsc -n "__fish_dsc_using_subcommand category; and not __fish_seen_subcommand_from list copy pull push help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c dsc -n "__fish_dsc_using_subcommand category; and __fish_seen_subcommand_from list" -s d -l discourse -r
complete -c dsc -n "__fish_dsc_using_subcommand category; and __fish_seen_subcommand_from list" -s h -l help -d 'Print help'
complete -c dsc -n "__fish_dsc_using_subcommand category; and __fish_seen_subcommand_from copy" -s d -l discourse -r
complete -c dsc -n "__fish_dsc_using_subcommand category; and __fish_seen_subcommand_from copy" -s h -l help -d 'Print help'
complete -c dsc -n "__fish_dsc_using_subcommand category; and __fish_seen_subcommand_from pull" -s d -l discourse -r
complete -c dsc -n "__fish_dsc_using_subcommand category; and __fish_seen_subcommand_from pull" -s h -l help -d 'Print help'
complete -c dsc -n "__fish_dsc_using_subcommand category; and __fish_seen_subcommand_from push" -s d -l discourse -r
complete -c dsc -n "__fish_dsc_using_subcommand category; and __fish_seen_subcommand_from push" -s h -l help -d 'Print help'
complete -c dsc -n "__fish_dsc_using_subcommand category; and __fish_seen_subcommand_from help" -f -a "list"
complete -c dsc -n "__fish_dsc_using_subcommand category; and __fish_seen_subcommand_from help" -f -a "copy"
complete -c dsc -n "__fish_dsc_using_subcommand category; and __fish_seen_subcommand_from help" -f -a "pull"
complete -c dsc -n "__fish_dsc_using_subcommand category; and __fish_seen_subcommand_from help" -f -a "push"
complete -c dsc -n "__fish_dsc_using_subcommand category; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c dsc -n "__fish_dsc_using_subcommand group; and not __fish_seen_subcommand_from list info copy help" -s h -l help -d 'Print help'
complete -c dsc -n "__fish_dsc_using_subcommand group; and not __fish_seen_subcommand_from list info copy help" -f -a "list"
complete -c dsc -n "__fish_dsc_using_subcommand group; and not __fish_seen_subcommand_from list info copy help" -f -a "info"
complete -c dsc -n "__fish_dsc_using_subcommand group; and not __fish_seen_subcommand_from list info copy help" -f -a "copy"
complete -c dsc -n "__fish_dsc_using_subcommand group; and not __fish_seen_subcommand_from list info copy help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c dsc -n "__fish_dsc_using_subcommand group; and __fish_seen_subcommand_from list" -s d -l discourse -r
complete -c dsc -n "__fish_dsc_using_subcommand group; and __fish_seen_subcommand_from list" -s h -l help -d 'Print help'
complete -c dsc -n "__fish_dsc_using_subcommand group; and __fish_seen_subcommand_from info" -s d -l discourse -r
complete -c dsc -n "__fish_dsc_using_subcommand group; and __fish_seen_subcommand_from info" -s g -l group -r
complete -c dsc -n "__fish_dsc_using_subcommand group; and __fish_seen_subcommand_from info" -s h -l help -d 'Print help'
complete -c dsc -n "__fish_dsc_using_subcommand group; and __fish_seen_subcommand_from copy" -s d -l discourse -r
complete -c dsc -n "__fish_dsc_using_subcommand group; and __fish_seen_subcommand_from copy" -s t -l target -r
complete -c dsc -n "__fish_dsc_using_subcommand group; and __fish_seen_subcommand_from copy" -s g -l group -r
complete -c dsc -n "__fish_dsc_using_subcommand group; and __fish_seen_subcommand_from copy" -s h -l help -d 'Print help'
complete -c dsc -n "__fish_dsc_using_subcommand group; and __fish_seen_subcommand_from help" -f -a "list"
complete -c dsc -n "__fish_dsc_using_subcommand group; and __fish_seen_subcommand_from help" -f -a "info"
complete -c dsc -n "__fish_dsc_using_subcommand group; and __fish_seen_subcommand_from help" -f -a "copy"
complete -c dsc -n "__fish_dsc_using_subcommand group; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c dsc -n "__fish_dsc_using_subcommand backup; and not __fish_seen_subcommand_from create list restore help" -s h -l help -d 'Print help'
complete -c dsc -n "__fish_dsc_using_subcommand backup; and not __fish_seen_subcommand_from create list restore help" -f -a "create"
complete -c dsc -n "__fish_dsc_using_subcommand backup; and not __fish_seen_subcommand_from create list restore help" -f -a "list"
complete -c dsc -n "__fish_dsc_using_subcommand backup; and not __fish_seen_subcommand_from create list restore help" -f -a "restore"
complete -c dsc -n "__fish_dsc_using_subcommand backup; and not __fish_seen_subcommand_from create list restore help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c dsc -n "__fish_dsc_using_subcommand backup; and __fish_seen_subcommand_from create" -s d -l discourse -r
complete -c dsc -n "__fish_dsc_using_subcommand backup; and __fish_seen_subcommand_from create" -s h -l help -d 'Print help'
complete -c dsc -n "__fish_dsc_using_subcommand backup; and __fish_seen_subcommand_from list" -s d -l discourse -r
complete -c dsc -n "__fish_dsc_using_subcommand backup; and __fish_seen_subcommand_from list" -s h -l help -d 'Print help'
complete -c dsc -n "__fish_dsc_using_subcommand backup; and __fish_seen_subcommand_from restore" -s d -l discourse -r
complete -c dsc -n "__fish_dsc_using_subcommand backup; and __fish_seen_subcommand_from restore" -s h -l help -d 'Print help'
complete -c dsc -n "__fish_dsc_using_subcommand backup; and __fish_seen_subcommand_from help" -f -a "create"
complete -c dsc -n "__fish_dsc_using_subcommand backup; and __fish_seen_subcommand_from help" -f -a "list"
complete -c dsc -n "__fish_dsc_using_subcommand backup; and __fish_seen_subcommand_from help" -f -a "restore"
complete -c dsc -n "__fish_dsc_using_subcommand backup; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c dsc -n "__fish_dsc_using_subcommand completions" -s d -l dir -r -F
complete -c dsc -n "__fish_dsc_using_subcommand completions" -s h -l help -d 'Print help'
complete -c dsc -n "__fish_dsc_using_subcommand help; and not __fish_seen_subcommand_from list add import update emoji topic category group backup completions help" -f -a "list"
complete -c dsc -n "__fish_dsc_using_subcommand help; and not __fish_seen_subcommand_from list add import update emoji topic category group backup completions help" -f -a "add"
complete -c dsc -n "__fish_dsc_using_subcommand help; and not __fish_seen_subcommand_from list add import update emoji topic category group backup completions help" -f -a "import"
complete -c dsc -n "__fish_dsc_using_subcommand help; and not __fish_seen_subcommand_from list add import update emoji topic category group backup completions help" -f -a "update"
complete -c dsc -n "__fish_dsc_using_subcommand help; and not __fish_seen_subcommand_from list add import update emoji topic category group backup completions help" -f -a "emoji"
complete -c dsc -n "__fish_dsc_using_subcommand help; and not __fish_seen_subcommand_from list add import update emoji topic category group backup completions help" -f -a "topic"
complete -c dsc -n "__fish_dsc_using_subcommand help; and not __fish_seen_subcommand_from list add import update emoji topic category group backup completions help" -f -a "category"
complete -c dsc -n "__fish_dsc_using_subcommand help; and not __fish_seen_subcommand_from list add import update emoji topic category group backup completions help" -f -a "group"
complete -c dsc -n "__fish_dsc_using_subcommand help; and not __fish_seen_subcommand_from list add import update emoji topic category group backup completions help" -f -a "backup"
complete -c dsc -n "__fish_dsc_using_subcommand help; and not __fish_seen_subcommand_from list add import update emoji topic category group backup completions help" -f -a "completions"
complete -c dsc -n "__fish_dsc_using_subcommand help; and not __fish_seen_subcommand_from list add import update emoji topic category group backup completions help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c dsc -n "__fish_dsc_using_subcommand help; and __fish_seen_subcommand_from emoji" -f -a "add"
complete -c dsc -n "__fish_dsc_using_subcommand help; and __fish_seen_subcommand_from topic" -f -a "pull"
complete -c dsc -n "__fish_dsc_using_subcommand help; and __fish_seen_subcommand_from topic" -f -a "push"
complete -c dsc -n "__fish_dsc_using_subcommand help; and __fish_seen_subcommand_from topic" -f -a "sync"
complete -c dsc -n "__fish_dsc_using_subcommand help; and __fish_seen_subcommand_from category" -f -a "list"
complete -c dsc -n "__fish_dsc_using_subcommand help; and __fish_seen_subcommand_from category" -f -a "copy"
complete -c dsc -n "__fish_dsc_using_subcommand help; and __fish_seen_subcommand_from category" -f -a "pull"
complete -c dsc -n "__fish_dsc_using_subcommand help; and __fish_seen_subcommand_from category" -f -a "push"
complete -c dsc -n "__fish_dsc_using_subcommand help; and __fish_seen_subcommand_from group" -f -a "list"
complete -c dsc -n "__fish_dsc_using_subcommand help; and __fish_seen_subcommand_from group" -f -a "info"
complete -c dsc -n "__fish_dsc_using_subcommand help; and __fish_seen_subcommand_from group" -f -a "copy"
complete -c dsc -n "__fish_dsc_using_subcommand help; and __fish_seen_subcommand_from backup" -f -a "create"
complete -c dsc -n "__fish_dsc_using_subcommand help; and __fish_seen_subcommand_from backup" -f -a "list"
complete -c dsc -n "__fish_dsc_using_subcommand help; and __fish_seen_subcommand_from backup" -f -a "restore"
