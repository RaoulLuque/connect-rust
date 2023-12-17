pub const START_PAGE_TEMPLATE: &'static str = r#"
<html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta http-equiv="X-UA-Compatible" content="IE=edge">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Connect-Rust</title>
    </head>
    {% if not over %}
        <body>
        {% if turn %}
            <h1>
                The enemy made a turn. The current state of the board is: 
            </h1>
            {{ turn.board_as_string }}
            {% endif %}

            <h1>What turn would you like to make?</h1>
            <br>
            <h2>Turn</h2>
            <form action="/" method="post">
                <!-- turn -->
                {% if turn %}
                <input type="hidden" name="current_gamestate" id="current_gamestate" value = {{ turn.current_gamestate_encoded }}>
                <br>
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
                <input type="text" name="engine" id="engine" value="random">
                <br> <br>
                <input type="submit" value="Submit">
            </form>
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
