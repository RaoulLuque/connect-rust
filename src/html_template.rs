pub const START_PAGE_TEMPLATE: &'static str = r#"
<html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta http-equiv="X-UA-Compatible" content="IE=edge">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Connect-Rust</title>
    </head>
    <style>
        span.a {
            width: 500px;
            height: px;
            padding: 10px;
            border: 5px solid gray;
            margin: 0;
        }
        h1 {
            font-family: 'Helvetica', 'Arial', sans-serif;
        }
        h2 {
            font-family: 'Helvetica', 'Arial', sans-serif;
        }
        p {
            font-family: 'Helvetica', 'Arial', sans-serif;
        }
    </style>
    <h1>
        <center> Connect-Rust </center>
    </h1>
    <h2>
        <center> Welcome to connect-rust. A connect-four game implemented completely in 
        <a href="https://www.rust-lang.org/">Rust</a> ! </center>
    </h2>
    
    <div> 

             <center>
                <p> 
                    Below you can play against one of the bots I've implemented. <br> 
                    Just choose which column  you'd like to play in order to make a turn and which bot you'd like to play against. <br> 
                    Currently the possible bots are: "Random", "Monte Carlo". 
                </p> 
            </center>

    </div>
    <center> <p>(This page looks horrible because programming is difficult) </p></center>  

    {% if not over %}
        <body>
            <h1>What turn would you like to make?</h1>
            <h2>Turn</h2>
            <form action="/" method="post">
                <!-- turn -->
                {% if turn %}
                <input type="hidden" name="current_gamestate" id="current_gamestate" value = {{ turn.current_gamestate_encoded }}>
                {% else %}
                <input type="hidden" name="current_gamestate" id="current_gamestate" value = 0>
                <br>
                {% endif %}
                <label for="column">Turn (Enter the number of the column you'd like to drop a token into as an arabic number)</label>
                <br>
                <input type="text" name="column" id="column">
                <br>
                <label for="engine">Engine (engine you'd like to play against)</label>
                <br>
                <input type="text" name="engine" id="engine" value="Monte Carlo">
                <br> <br>
                <input type="submit" value="Submit">
            </form>
            {% if turn %}
            <h1>
                The enemy made a turn. The current state of the board is: 
            </h1>
            {{ turn.board_as_string }}
            {% endif %}
        </body>
    {% else %}
        <h1>
            The game is over. The current state of the board is:
        </h1>
        {{ turn.board_as_string }}

        <h1> And {{ turn.who_won }} won. Congratulations! </h1>
    {% endif %}
</html>
"#;
