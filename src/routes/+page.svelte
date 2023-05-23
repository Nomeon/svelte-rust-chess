<script>
    import { onMount } from "svelte";
    import { draggable } from '@neodrag/svelte';
    import { checkMove } from '../Logic.svelte';
    import init, { greet } from 'rust';

    // To get WASM working:
    // https://www.reddit.com/r/sveltejs/comments/vzf86d/sveltekit_with_webassembly_rust/
    // https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm


    $: board = [];

    onMount(async() => {
        await init();
        // greet("Svelte");
        board = createBoard();
        initializeBoard();
        console.log(board);
    })

    function createBoard() {
        let board = new Array(8);
        for (let i = 0; i < board.length; i++) {
            board[i] = new Array(8);
            for (let j = 0; j < board[i].length; j++) {
                board[i][j] = {
                    id: i * 8 + j,
                    row: i,
                    col: j,
                    piece: null,
                    color: undefined,
                    draggable: false,
                    possibleMoves: [],
                };
            }
        }
        return board;
    }
    
    function initializeBoard() {
        board[0][0].piece = "rook";
        board[0][1].piece = "knight";
        board[0][2].piece = "bishop";
        board[0][3].piece = "queen";
        board[0][4].piece = "king";
        board[0][5].piece = "bishop";
        board[0][6].piece = "knight";
        board[0][7].piece = "rook";
        for (let i = 0; i < board[1].length; i++) {
            board[1][i].piece = "pawn";
        }

        board[7][0].piece = "rook";
        board[7][1].piece = "knight";
        board[7][2].piece = "bishop";
        board[7][3].piece = "queen";
        board[7][4].piece = "king";
        board[7][5].piece = "bishop";
        board[7][6].piece = "knight";
        board[7][7].piece = "rook";
        for (let i = 0; i < board[6].length; i++) {
            board[6][i].piece = "pawn";
        }

        for (let h = 0; h < board[0].length; h++) {
            board[0][h].color = "white";
            board[1][h].color = "white";
            board[6][h].color = "black";
            board[7][h].color = "black";
        }
    }

    function dropPiece(e, square) {
        let y = e.detail.offsetY;
        let x = e.detail.offsetX;

        if (!((y <= 25 && y >= 0) || (y >= -25) && (y <= 0)) || !((x <= 25 && x >= 0) || (x >= -25) && (x <= 0))) {
            let newXY = [Math.round(x/50), Math.round(y/50)];

            if (checkMove(board, square, newXY)) {
                board[square.row + newXY[1]][square.col + newXY[0]].piece = square.piece;
                board[square.row + newXY[1]][square.col + newXY[0]].color = square.color;
                square.piece = null;
                square.color = undefined;
                return;
            }
        } 
        e.detail.rootNode.style.transform = "translate(0px, 0px)";
    }


</script>
<table class='board'>
    <tbody>
        {#each board as row}
        <tr>
            {#each row as square}
            <td class="square {((square.row % 2) + square.id) % 2 === 0 ? 'white' : 'black'}">
                {#if square.piece !== null}
                <div use:draggable on:neodrag:end={(e) => dropPiece(e, square)} class='piece-square'>
                    <img src={`/pieces/${square.color}_${square.piece}.png`} alt="{square.piece}" class="piece-image"/>
                </div>
                {/if}
            </td>
            {/each}
        </tr>
        {/each}
    </tbody>
</table>

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