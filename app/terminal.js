export class Terminal {
    constructor(inputEl, outputEl) {
        this.inputEl = inputEl;
        this.outputEl = outputEl;

        this.state = {
            id: 'idle',
        };

        this.history = [];

        this.inputEl.onkeyup = (event) => {
            switch (event.keyCode) {
                case 13:
                    this._handleEnter(event);
                    break;

                case 38:
                    this._handleArrowUp(event);
                    break;

                case 40:
                    this._handleArrowDown(event);
                    break;

                default:
                    if (this.state.id == 'browsing-history') {
                        this.state = {
                            id: 'idle',
                        };
                    }
            }
        };

        this.onInputHandler = (_line) => void 0;
    }

    onInput(fn) {
        this.onInputHandler = fn;
    }

    println(msg) {
        if (this.outputEl.value) {
            this.outputEl.value += '\n';
        }

        this.outputEl.value += msg;
        this.outputEl.scrollTop = this.outputEl.scrollHeight;
    }

    scrollToTop() {
        this.inputEl.focus();
        this.outputEl.scrollTop = 0;
    }

    _handleEnter(event) {
        event.preventDefault();

        const input = this.inputEl.value.trim();

        if (input.length > 0) {
            this.history.push(input);
            this.onInputHandler(input);
        }

        this.inputEl.value = '';

        this.state = {
            id: 'idle',
        };
    }

    _handleArrowUp(event) {
        event.preventDefault();

        switch (this.state.id) {
            case 'idle':
                if (this.history.length == 0) {
                    return;
                }

                if (this.inputEl.value.length > 0) {
                    return;
                }

                this.state = {
                    id: 'browsing-history',
                    historyIdx: 1,
                };

                this.inputEl.value = this.history[this.history.length - 1];
                break;

            case 'browsing-history':
                if (this.state.historyIdx < this.history.length) {
                    this.state.historyIdx += 1;

                    this.inputEl.value = this.history[
                    this.history.length - this.state.historyIdx
                        ];
                } else {
                    this.inputEl.value = '';
                }

                break;
        }
    }

    _handleArrowDown(event) {
        event.preventDefault();

        if (this.state.id == 'browsing-history') {
            if (this.state.historyIdx > 1) {
                this.state.historyIdx -= 1;

                this.inputEl.value = this.history[
                this.history.length - this.state.historyIdx
                    ];
            } else {
                this.inputEl.value = '';
            }
        }
    }

    intro_message(config) {
        this.println("");
        this.println("Simulation of evolution, powered by neural network, genetic algorithm & high-school math.");
        this.println("");
        this.println("Modified from a tutorial found at https://pwy.io/en/posts/learning-to-fly-pt1/");
        this.println("Original source code: https://github.com/Patryk27/shorelark");
        this.println("");
        this.println("---- About ----");
        this.println("");
        this.println("Each triangle represents a plankton; each bird has an *eye*, whose eyesight is drawn around the plankton" +
            ", and a *brain* that decides where and how fast the plankton should be moving.");
        this.println("");
        this.println("Each circle represents a food, for our plankton this would be plastic or carbon");
        this.println("");
        this.println("All plankton start flying with randomized brains, and after 2500 turns (around 40 real-time seconds), agents who managed to eat the most foods are reproduced, and their offspring starts the simulation anew.");
        this.println("");
        this.println("You can affect the simulation by entering commands in the input at the bottom of this box - for starters, try executing the `train` command a few times (write `t`, press enter, write `t`, press enter etc.) - this fast-forwards the simulation, allowing you to see the birds getting smarter by the second.");
        this.println("");
        this.println("Have fun!");
        this.println("");
        this.println("---- Commands ----");
        this.println("");
        this.println("- p / pause");
        this.println("  Pauses (or resumes) the simulation");
        this.println("");
        this.println(`- r / reset [animals=${config.world_animals}] [f=${config.world_foods}] [...]`);
        this.println("  Starts simulation from scratch with given optional");
        this.println("  parameters:");
        this.println("");
        this.println(`  * a / animals (default=${config.world_animals})`);
        this.println("    number of animals");
        this.println("");
        this.println(`  * f / foods (default=${config.world_foods})`);
        this.println("    number of foods");
        this.println("");
        this.println(`  * n / neurons (default=${config.brain_neurons})`);
        this.println("    number of brain neurons per each animal");
        this.println("");
        this.println(`  * p / photoreceptors (default=${config.eye_cells})`);
        this.println("    number of eye cells per each animal");
        this.println("");
        this.println("  Examples:");
        this.println("    reset animals=100 foods=100");
        this.println("    r a=100 f=100");
        this.println("    r p=3");
        this.println("");
        this.println("- (t)rain [how-many-generations]");
        this.println("  Fast-forwards one or many generations, allowing to");
        this.println("  observe the learning process faster.");
        this.println("");
        this.println("  Examples:");
        this.println("    train");
        this.println("    t 5");
        this.println("");
        this.println("---- Advanced Tips™ ----");
        this.println("");
        this.println("r i:ga_alg=(1,2,3)");
        this.println("\t1-RouletteSelection\n\t2-TournamentSelection");
        this.println("");
        this.println("- `reset` can modify *all* of the parameters:");
        this.println("");
        this.println("  * r i:integer_param=123 f:float_param=123");
        this.println("  * r a=200 f=200 f:food_size=0.002");
        this.println("");
        this.println("---- Funky scenarios ----");
        this.println("");
        this.println("  * r i:ga_reverse=1 f:sim_speed_min=0.003");
        this.println("    (birdies *avoid* food)");
        this.println("");
        this.println("  * r i:brain_neurons=1");
        this.println("    (single-neuroned zombies)");
        this.println("");
        this.println("  * r f:food_size=0.05");
        this.println("    (biiiigie birdies)");
        this.println("");
        this.println("  * r f:eye_fov_angle=0.45");
        this.println("    (narrow field of view)");
        this.println("");
        this.println("----");
    }
}
