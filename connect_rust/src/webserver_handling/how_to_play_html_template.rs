pub const HOW_TO_PLAY_TEMPLATE: &'static str = r#"
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

            /* Lists */
            ul {
                margin-top: 1%;
            }

            ul li {
                margin-bottom: 1%;
            }

            dl {
                margin-top: 2%;
            }

            dl dt {
                margin-bottom: 0.1%;
            }

            dl dd {
                margin-bottom: 1.5%;
                text-align: justify;
            }

            /* Images */
            .git_icon {
                position: fixed; 
                top: 3%;
                right: 3%;
                width: 8%;
                aspect-ratio: 230/225;
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
            .explanation_text {
                font-family: 'Helvetica', 'Arial', sans-serif;
                font-size: medium;
                text-align: left;
                width: 90%;
                display: block;
                margin: auto;
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

            /* Lists */
            ul {
                margin-top: 1%;
            }

            ul li {
                margin-bottom: 1%;
            }

            dl {
                margin-top: 2%;
            }

            dl dt {
                margin-bottom: 0.1%;
            }

            dl dd {
                margin-bottom: 1.5%;
            }

            .engine_row {
                display: flex;
                padding-bottom: 2%;
            }
            .engine {
                position: relative;
                width: 50%;
            }
            .engine_left {
                padding-right: 5%;
            }

            /* Images */
            .git_icon {
                position: fixed; 
                top: 3%;
                right: 2%;
                width: 5%;
                aspect-ratio: 230/225;
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
            .explanation_text {
                font-family: 'Helvetica', 'Arial', sans-serif;
                font-size: x-large;
                text-align: left;
                width:90%;
                display: block;
                margin: auto;
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
        <link rel="icon" href="https://i.ibb.co/HnKC9Vp/connect-rust.png">
    </head>

    <!-- Git Icon -->
    <a href="https://github.com/RaoulLuque/connect-rust">
        <img src="https://i.ibb.co/MBWwy25/github-mark-white.png" alt="GitHub Page" class="git_icon"/>
    </a>

    <div class="center">
        <h1>
            How to <a href="/">play</a>
        </h1>
        <div class="explanation_text">
            The game is connect four. You can play as follows:
            <ul>
                <li>Each turn pick a column to play in by entering a number into the "Column" field</li>
                <li>Choose the bot/engine you wish to play against and click "Make Turn"</li>
                <li>The bot will calculate an answer and you will see the board with the bots turn made</li>
            </ul>  <br> <br>

            The rules are:
            <ul>
                <li>Columns may be played if there is space left in the column</li>
                <li>The human/user and the bot alternate in playing tokens</li>
                <li>The player that achieves to get a row, column or diagonal of their color wins</li>
                <li>The human/user plays as blue and starts the game</li>
            </ul> <br> <br>

            The different bots are:
            <dl>
                <div class="engine_row">
                    <div class="engine">
                        <dt>Bruteforce</dt>
                            <dd> Calculates the next move taking into consideration all possible next gamestates</dd>
                    </div>
                    <div class="engine">
                        <dt>Random*</dt>
                            <dd> Plays randomly except when there are three in a row for the human. In which case the fourth token is placed to avoid loosing</dd>
                    </div>
                    <div class="engine">
                        <dt>Bruteforce N%</dt>
                            <dd> Plays as Bruteforce N% of the time. Otherwise plays as Random*</dd>
                    </div>
                </div>
                <div class="engine_row">
                    <div class="engine">
                        <dt>Monte Carlo</dt>
                            <dd> Semi-randomly simulates as much games as possible from the current gamestate and chooses the next gamestates according to which was best in the simulations</dd>
                    </div>
                    <div class="engine">
                        <dt>Random</dt>
                            <dd> Plays completely random</dd>
                    </div>
                    <div class="engine">
                        <dt></dt>
                            <dd> </dd>
                    </div>
                </div>
            </dl>

            Note that due to the fact that a smooth game flow wants to be guaranteed, bruteforce uses 
            a pre-generated table to look up the first three turns. The displayed
            calculation times are the times the engine would take if it calculated the moves on the 
            fly. The engine calculates the moves on the fly from the fourth turn on which results in 
            the longer waiting times. <br> <br>

            For more information about the underlying techniques feel free to visit the <a href="https://github.com/RaoulLuque/connect-rust">GitHub Repository</a>.
        </div>
        <h2>
            Enough of the talking, I want to <a href="/">play</a>!
        </h2>     
    </div>
"#;
