pub const START_PAGE_TEMPLATE: &'static str = r#"
<html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta http-equiv="X-UA-Compatible" content="IE=edge">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Connect-Rust</title>
    </head>
    <body>
    <h1>
        The bots last turn was:
        {{ turn.column }}
    <h1

    <h1>What turn would you like to make?</h1>
        <br>
        <h2>Turn</h2>
        <form action="/" method="post">
            <!-- turn -->
            <label for="column">Turn (Enter the number of the column you'd like to drop a token into as an arabic number)</label>
            <br>
            <input type="text" name="column" id="column">
            <br> <br>
            <input type="submit" value="Submit">
        </form>
    </body>
</html>
"#;
