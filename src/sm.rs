use paste::paste;

macro_rules! sm {
(
    $name:ident :
    $(
        state $state_name:ident {
            start $start_block:block,
            execute $execute_block:block,
            next $next_block:block,
            end $end_block:block
        }
    )*
) => {
        paste! {
            #[derive(PartialEq, Copy, Clone)]
            enum [< $name States >] {
                $(
                    $state_name,
                )*
            }

            struct $name {
                current_state: [< $name States >],
                prev_state: [< $name States >],
            }
            impl $name {
                $(
                    fn [< state_ $state_name _start >] (&mut self) $start_block
                    fn [< state_ $state_name _execute >] (&mut self) $execute_block
                    fn [< state_ $state_name _next >] (&mut self) -> $next_block
                    fn [< state_ $state_name _end >] (&mut self) $end_block
                )*

                fn step(&mut self) {
                    if (self.prev_state != self.current_state) {
                        match &self.current_state {
                            $(
                                [< $name States >]::$state_name => self.[< state_ $state_name _start >](),
                            )*
                        }
                        self.prev_state = self.current_state;
                    }
                    
                    match &self.current_state {
                        $(
                            [< $name States >]::$state_name => {
                                self.[< state_ $state_name _execute >]();
                                self.current_state = self.[< state_ $state_name _next >]();
                                if self.current_state != self.prev_state {
                                    self.[< state_ $state_name _end >]();
                                }
                            },
                        )*
                    }
                }
            }
        }
    };
}

sm!(Foo :
    state A {
    start {

    },
    execute {
        5;
    },
    next {

    },
    end {

    }
});
