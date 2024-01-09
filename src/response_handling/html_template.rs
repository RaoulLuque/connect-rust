pub const START_PAGE_TEMPLATE: &'static str = r#"
<!DOCTYPE html>
<html lang="en">
    <style>
        body {
            background-color:  	#36454F;
            margin: 0;
            padding: 0;
        }
        * {
            color: #ffffff;
        }
        /* Game board */
        .game_board { 
            margin: auto;
            margin-top: 5%;
            display: grid;
            width: 50%;
            grid-template-columns: 1fr 1fr 1fr 1fr 1fr 1fr 1fr;
            justify-content: center;
        }  
        .box_white {
            background-color: #d9d9d9;
            border: 1px solid black;
            aspect-ratio: 1;
        }

        .box_blue {
            background-color: #303245;
            border: 1px solid black;
            aspect-ratio: 1;
        }

        .box_red {
            background-color: #b7152d;
            border: 1px solid black;
            aspect-ratio: 1;
        }

        select option {
            background: #303245;
        }

        /* Phone design */
        @media (min-width:0px) {
            .center {
                margin: auto;
                width: 70%;
                border: 0px;
                padding: 3%;
                text-align: center;
            }
            .entering_moves {
                display: block;
                text-align: center;
            }
            form {
                padding: 10px;
                width: 75%;
                display: inline-block;
                margin: auto;
            }
            .form {
                font-family: 'Helvetica', 'Arial', sans-serif;
                background-color: #15172b;
                border-radius: 20px;
                box-sizing: border-box;
                height: 500px;
                padding: 20px;
                width: 320px;
            }
            /* Column and engine buttons */
            .form_row {
                display: flex;
            }
            .form_field {
                height: 40px;
                position: relative;
                width: 50%;
            }
            .form_field_left {
                padding-right: 5%;
            }
            .input {
                background-color: #303245;
                border-radius: 12px;
                border: 0;
                box-sizing: border-box;
                color: #ffffff;
                font-size: small;
                height: 100%;
                outline: 0;
                padding: 4px 20px 0;
                width: 100%;
            }
            .cut {
                background-color: #36454F;
                border-radius: 10px;
                height: 20px;
                left: 7%;
                position: absolute;
                top: -20px;
                transform: translateY(0);
                transition: transform 200ms;
                width: 76px;
            }
            .cut-short {
                width: 50px;
            }
            .input:focus ~ .cut,
            .input:not(:placeholder-shown) ~ .cut {
                transform: translateY(8px);
            }

            .placeholder {
                color: #36454F;
                font-size: medium;
                left: 7%;
                line-height: 50%;
                pointer-events: none;
                position: absolute;
                transform-origin: 0 50%;
                transition: transform 200ms, color 200ms;
                top: 40%;
            }

            .input:focus ~ .placeholder,
            .input:not(:placeholder-shown) ~ .placeholder {
                transform: translateY(-25px) translateX(10px) scale(0.75);
            }

            .input:not(:placeholder-shown) ~ .placeholder {
                color: #ffffff;
            }

            .input:focus ~ .placeholder {
                color: #ffffff;
            }

            /* Make move button */
            .submit {
                background-color: #36454F;
                border-radius: 12px;
                border: 0;
                box-sizing: border-box;
                cursor: pointer;
                font-size: medium;
                height: 40px;
                margin-top: 2%;
                margin-bottom: 5%;
                outline: 0;
                text-align: center;
                width: 100%;
            }

            .submit:active {
                background-color: #303245;
            }

            /* Wrapper for game message and board */
            .game_wrapper {
                border: solid;
                border-color: #ffffff;
                margin: auto;
                width: 70%;
                padding: 3%;
                border-radius: 20px;
                align-items: center;
                justify-content: center;
            }

            /* Text */
            h1 {
                font-family: "Raleway", sans-serif;
                font-size: xx-large;
            }
            h2 {
                font-family: 'Helvetica', 'Arial', sans-serif;
                font-size: medium;
            }
            p {
                font-family: 'Helvetica', 'Arial', sans-serif;
            }
            /* Current state */
            .game_text {
                font-family: 'Helvetica', 'Arial', sans-serif;
                font-size: small;
                margin: auto;
                width: 100%;
                text-align: left;
            }
        }

        /* ---------------------------- Desktop design ---------------------------- */
        @media (min-width:800px) {
            .center {
                margin: auto;
                width: 70%;
                border: 0px;
                padding: 2%;
                text-align: center;
            }
            .entering_moves {
                display: block;
                text-align: center;
            }
            form {
                padding: 10px;
                width: 47%;
                display: inline-block;
                margin: auto;
            }
            .form {
                font-family: 'Helvetica', 'Arial', sans-serif;
                background-color: #15172b;
                border-radius: 20px;
                box-sizing: border-box;
                height: 500px;
                padding: 20px;
                width: 320px;
            }
            /* Column and engine buttons */
            .form_row {
                display: flex;
            }
            .form_field {
                height: 50px;
                position: relative;
                width: 50%;
            }
            .form_field_left {
                padding-right: 5%;
            }
            .input {
                background-color: #303245;
                border-radius: 12px;
                border: 0;
                box-sizing: border-box;
                color: #ffffff;
                font-size: 18px;
                height: 100%;
                outline: 0;
                padding: 4px 20px 0;
                width: 100%;
            }
            .cut {
                background-color: #36454F;
                border-radius: 10px;
                height: 20px;
                left: 7%;
                position: absolute;
                top: -20px;
                transform: translateY(0);
                transition: transform 200ms;
                width: 76px;
            }
            .cut-short {
                width: 50px;
            }
            .input:focus ~ .cut,
            .input:not(:placeholder-shown) ~ .cut {
                transform: translateY(8px);
            }

            .placeholder {
                color: #36454F;
                font-size: large;
                left: 7%;
                line-height: 50%;
                pointer-events: none;
                position: absolute;
                transform-origin: 0 50%;
                transition: transform 200ms, color 200ms;
                top: 40%;
            }

            .input:focus ~ .placeholder,
            .input:not(:placeholder-shown) ~ .placeholder {
                transform: translateY(-30px) translateX(10px) scale(0.75);
            }

            .input:not(:placeholder-shown) ~ .placeholder {
                color: #ffffff;
            }

            .input:focus ~ .placeholder {
                color: #ffffff;
            }

            /* Make move button */
            .submit {
                background-color: #36454F;
                border-radius: 12px;
                border: 0;
                box-sizing: border-box;
                cursor: pointer;
                font-size: 18px;
                height: 50px;
                margin-top: 2%;
                margin-bottom: 5%;
                outline: 0;
                text-align: center;
                width: 100%;
            }

            .submit:active {
                background-color: #303245;
            }

            /* Wrapper for game message and board */
            .game_wrapper {
                border: solid;
                border-color: #ffffff;
                margin: auto;
                width: 50%;
                padding: 3%;
                border-radius: 20px;
                align-items: center;
                justify-content: center;
            } 

            /* Text */
            h1 {
                font-family: "Raleway", sans-serif;
                font-size: xxx-large;
            }
            h2 {
                font-family: 'Helvetica', 'Arial', sans-serif;
                font-size: large;
            }
            p {
                font-family: 'Helvetica', 'Arial', sans-serif;
            }
            /* Current state */
            .game_text {
                font-family: 'Helvetica', 'Arial', sans-serif;
                font-size: large;
                margin: auto;
                width: 100%;
                text-align: left;
                font-size: large;
            }
        }
    </style>
    <head>
        <meta charset="UTF-8">
        <meta http-equiv="X-UA-Compatible" content="IE=edge">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Connect-Rust</title>
        <meta name="description" content="A connect-four game implemented in Rust">
        <link rel="stylesheet" href="https://fonts.googleapis.com/css?family=Raleway">
    </head>
    <div class="center">
        <h1>
            Connect-Rust &#129408
        </h1>
        <h2>
            A connect-four game implemented completely in <a href="https://www.rust-lang.org/">Rust</a>! Here's how to <a href="how-to-play">play</a>
        </h2>
    </div>
    {% if not over %}
    <!-- Form for entering next move -->
    <div class="entering_moves">
        <form action="/" method="post">
            {% if turn %}
                <input type="hidden" name="current_and_previous_gamestates" id="current_and_previous_gamestates" value = {{ turn.boards_as_string }}>
            {% else %}
                <input type="hidden" name="current_and_previous_gamestates" id="current_and_previous_gamestates" value = "">
                <br>
            {% endif %}

            <div class="form_row">
                <div class="form_field form_field_left">
                    <input id="column" name="column" class="input" type="text" placeholder=" " />
                    <div class="cut"></div>
                    <label for="column" class="placeholder">Column</label>
                </div>
                <div class="form_field">
                    <select name="engine" id="engine" class="input">
                        <option value="Bruteforce" {% if bruteforce %} selected {% endif %}>Bruteforce</option>
                        <option value="Bruteforce 75%" {% if bruteforce_seventy_five_percent %} selected {% endif %}>Bruteforce 75%</option>
                        <option value="Bruteforce 50%" {% if bruteforce_fifty_percent %} selected {% endif %}>Bruteforce 50%</option>
                        <option value="Bruteforce 25%" {% if bruteforce_twenty_five_percent %} selected {% endif %}>Bruteforce 25%</option>
                        <option value="Random*" {% if random_glowed_up %} selected {% endif %}>Random*</option>
                        <option value="Monte Carlo" {% if monte_carlo %} selected {% endif %}>Monte Carlo</option>
                        <option value="Random"{% if random %} selected {% endif %}>Random</option>
                    </select>
                    <div class="cut"></div>
                    <label for="engine" class="placeholder"> Engine</label>
                </div>
            </div>
            <button type="submit" class="submit">Make Turn</button>
          </form>
    </div>

    <div class="game_wrapper">
        {% if turn %}
            <div class="game_text">
                The enemy made a turn. The current state of the board is:  <br>
            </div>
            <!-- Printing the previous gamestates -->
            {% for board in turn.boards %}
                {% if loop.first %}
                    {% if turn.move_was_invalid %}
                        <div class="game_text">
                            Your last move was invalid. We chose the last possible column <br>
                        </div>
                    {% endif %}

                    <div class="game_board">
                        {{ board }} <br> <br>
                    </div> <br>

                    <div class="game_text">
                        The computation took: {{ turn.computation_time }} microseconds (1.000.000th of a second).
                        While computing the move, the bot visited {{ turn.number_of_visited_nodes }} nodes in order to find the best response. <br>
                    </div>

                    <div class="game_text">
                        <br> The previous states of the board were: <br>
                    </div>

                {% else %}
                    <div class="game_board">
                       {{ board }} <br> <br>
                    </div>
                {% endif %}
            {% endfor %}

        {% else %}
            <div class="game_text">
                The board is currently empty. The current state of the board is: <br>
            </div>

            <div class="game_board">
                {{ empty_gamestate_as_string_for_web }}
            </div>

        {% endif %}
    </div>
    {% else %}
    <div class="game_wrapper">
        <div class="game_text">
            The game is over. The current state of the board is: <br>
        </div>

        <div class="game_board">
            {{ turn.final_board_as_string }}
        </div>
        <h1> And {{ turn.who_won }} won. Congratulations! </h1>
    </div>
    {% endif %}
</html>
"#;
