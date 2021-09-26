from typing import Tuple


case1 = """\
.........................+.........+..............
......+..................+.........+..............
......+..................+.........+.....+........
......+..................+.........+.....+........
......+..................+.........+.....+........
##################################################
...........+......+..........+...............+....
...........+......+..........+...............+....
...........+......+..........+...............+....
...........+......+..........+...............+....
...........+.................................+...."""

matrix = [list(l) for l in case1.split('\n')]

MAINLINE = 5


def check_if_there_is_a_branch(matrix, j) -> int:
    """Checks if there is a branch starting at j.
    Returns -1 if it's upward, 1 if it's downward, or 0 if there is none.

    Args:
        matrix (list[list]): The matrix to check.
        j (int): The column to check.
    """
    if j < 0 or j >= len(matrix[0]):
        return 0
    if matrix[MAINLINE - 1][j] == '+':
        return -1
    if matrix[MAINLINE + 1][j] == '+':
        return 1
    return 0


def follow_branch(matrix, j, i=MAINLINE, last_position: Tuple[int, int] = None) -> Tuple[int, int]:
    """Follow a branch 'till the end and returns the end point.

    Args:
        matrix (list[list]): The matrix.
        j (int): The column to start.
        i (int): The row to start. (optional)
        last_position Tuple[int, int]: The last position. (optional)

    Returns:
        tuple(int, int): The end point (i, j).
    """
    for Δi, Δj in [(-1, 0), (1, 0), (0, 1), (0, -1)]:
        i_ = i + Δi
        j_ = j + Δj
        if last_position is not None and (i_, j_) == last_position:
            continue
        if i_ < 0 or i_ >= len(matrix):
            continue
        if j_ < 0 or j_ >= len(matrix[0]):
            continue
        if matrix[i_][j_] == '+':
            return follow_branch(matrix, j_, i_, (i, j))
    return (i, j)


def solve(matrix):
    """Solves the problem.

    Args:
        matrix (list[list]): The matrix.
    """
    results = []
    for j in range(1, 50):
        branch = check_if_there_is_a_branch(matrix, j)
        if branch != 0:
            end = follow_branch(matrix, j)
            nexus = end[0] == 0 or end[0] == len(matrix) - 1 or end[1] == 0 or end[1] == len(matrix[0]) - 1
            results.append((j, nexus))
    return results


def main():
    matrix = [list(input()) for i in range(11)]

    results = solve(matrix)
    for i, nexus in results:
        status = "Evento Nexus" if nexus else "Instavel"
        print(f"Bifurcacao {i}: {status}")


if __name__ == '__main__':
    main()
