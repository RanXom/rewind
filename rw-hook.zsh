rw_preexec() {
    export RW_LAST_CMD="$1"
    export RW_START_TIME=$(date +%s)
}

rw_precmd() {
    local EXIT_CODE=$?
    if [ -n "$RW_LAST_CMD" ]; then
        # Call binary with flags
        rw log \
           --command "$RW_LAST_CMD" \
           --exit "$EXIT_CODE" \
           --cwd "$PWD" \
           --start "$RW_START_TIME"

        # Clear it so it doesn't log on empty 'Enter' presses
        unset RW_LAST_CMD
    fi
}

# Register the hooks
autoload -Uz add-zsh-hook
add-zsh-hook preexec rw_preexec
add-zsh-hook precmd rw_precmd
