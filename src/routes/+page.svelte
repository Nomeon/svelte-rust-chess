<script>
    import { onMount } from "svelte";
    import { draggable } from '@neodrag/svelte';
    import init, { get_possible_moves, play_move } from 'rust';

    // To get WASM working:
    // https://www.reddit.com/r/sveltejs/comments/vzf86d/sveltekit_with_webassembly_rust/
    // https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm

    $: board = [];
    $: turn = "";
    $: boardFEN = "";

    const files = ["a", "b", "c", "d", "e", "f", "g", "h"];
    const ranks = [8, 7, 6, 5, 4, 3, 2, 1];   

    onMount(async() => {
        await init();
        initializeBoard();
    })

    function initializeBoard() {
        for (let i = 0; i < 64; i++) {
            let rank = ranks[Math.floor(i / 8)];
            let file = files[i % 8];
            let square = file + String(rank);
            let piece = "";
            let id = i;
            board[i] = {rank: rank, file: file, piece: piece, id: id, square: square, draggable: true, color: ""};
        }
        boardFEN = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        fenToBoard(boardFEN);
    }

    function fenToBoard(fen) {
        let fenParts =  fen.split(" ");
        let fenPart1 = fenParts[0]
        let pieces = fenPart1.split("/").join("");
        let fake_board = [];

        for (let i = 0; i < pieces.length; i++) {
            let char = pieces[i];
            if (!isNaN(char)) {
                for (let j = 0; j < parseInt(char); j++) {
                    fake_board.push("");
                }
            }
            else {
                fake_board.push(char);
            }
        }
        for (let i = 0; i < 64; i++) {
            board[i].piece = fake_board[i];
        }
        // fenParts[1] === "w" ? setDraggable("b") : setDraggable("w");
    }

    function setDraggable(turnVar) {
        turn = turnVar;
        if (turn === "w") {
            board.forEach((square) => {
                if (square.piece === square.piece.toUpperCase()) {
                    square.draggable = true;
                }
                else {
                    square.draggable = false;
                }
            })
        }
        else {
            board.forEach((square) => {
                if (square.piece === square.piece.toUpperCase()) {
                    square.draggable = true;
                }
                else {
                    square.draggable = false;
                }
            })
        }
    }

    function pickPiece(e, square) {
        let moves = get_possible_moves(boardFEN, square.square);
        showMoves(moves);
    }

    function dropPiece(e, square) {
        let y = e.detail.offsetY;
        let x = e.detail.offsetX;
        showMoves([]);

        if (!((y <= 25 && y >= 0) || (y >= -25) && (y <= 0)) || !((x <= 25 && x >= 0) || (x >= -25) && (x <= 0))) {
            let newX = files[square.file.charCodeAt(0) - 97 + Math.round(x/50)];
            let newY = ranks[ranks.indexOf(square.rank) + Math.round(y/50)];
            let target = newX + String(newY);
            let moves = get_possible_moves(boardFEN, square.square);
            if (moves.includes(target)) {
                boardFEN = play_move(boardFEN, square.square, target);
                fenToBoard(boardFEN);
                console.log(board)
                return;
            }
        } 
        e.detail.rootNode.style.transform = "translate(0px, 0px)";
    }

    function showMoves(moves) {
        if (moves.length > 0) {
            moves.forEach((move) => {
                document.getElementById(move).style.boxShadow = "200px 200px 200px rgba(0, 255, 0, 0.5) inset";
            })
        }
        else { 
            for (let i = 0; i < 64; i++) {
                let file = String.fromCharCode(97 + (i % 8)); // Convert the index to a file character (a-h)
                let rank = 8 - Math.floor(i / 8); // Convert the index to a rank number (1-8)
                let index = file + rank;
                document.getElementById(index).style.boxShadow = "none";
            }
        }
    }

</script>
{#if board.length > 0}
<table class='board'>
    <tbody>
        {#each ranks as rank, i}
        <tr>
            {#each files as file, j}
            <td class="square {((files.indexOf(file) % 2 + ranks.indexOf(rank))) % 2 === 0 ? 'white' : 'black'}" id={board[i * 8 + j].square}>
                {#if board[i * 8 + j].draggable && board[i * 8 + j].piece !== ""}
                    <div class='piece-square' use:draggable={{ bounds: 'table' }} on:neodrag:start={(e) => pickPiece(e, board[i * 8 + j])} on:neodrag:end={(e) => dropPiece(e, board[i * 8 + j])}>
                        <img src={`/pieces/${board[i * 8 + j].piece === board[i * 8 + j].piece.toUpperCase() ? "white" : "black"}_${board[i * 8 + j].piece}.png`} alt="{board[i * 8 + j].piece}" class="piece-image"/>
                    </div>
                {:else if board[i * 8 + j].piece !== ""}
                    <div class='piece-square'>
                        <img src={`/pieces/${board[i * 8 + j].piece === board[i * 8 + j].piece.toUpperCase() ? "white" : "black"}_${board[i * 8 + j].piece}.png`} alt="{board[i * 8 + j].piece}" class="piece-image"/>
                    </div>
                {/if}
            </td>
            {/each}
        </tr>
        {/each}
    </tbody>
</table>
{/if}

<style>
    .board {
        border-collapse: collapse;
        border: 2px solid black;
        margin: 0 auto;
    }

    .square {
        width: 50px;
        height: 50px;
        max-width: 50px;
        max-height: 50px;
        border: 1px solid black;
        text-align: center;
        vertical-align: middle;
        padding: 0;
        box-sizing: border-box;
    }

    .piece-square {
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .white {
        background-color: white;
    }

    .black {
        background-color: black;
        color: white;
    }

    .piece-image {
        max-width: 35px;
        max-height: 35px;
    }
</style>