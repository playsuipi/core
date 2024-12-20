/**
 * Copyright 2024 Macuyler Dunn <dev@macuyler.com>
 */

#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>

#include "../../target/playsuipi_core.h"

#define MAX_PILE_SIZE 52
#define HAND_SIZE 8
#define FLOOR_SIZE 13
#define CLUBS "♣"
#define DIAMONDS "♦"
#define HEARTS "♥"
#define SPADES "♠"

char *suit(unsigned char suit) {
    char *out;
    switch (suit) {
    case 0:
        out = CLUBS;
        break;
    case 1:
        out = DIAMONDS;
        break;
    case 2:
        out = HEARTS;
        break;
    case 3:
        out = SPADES;
        break;
    default:
        out = "_";
        break;
    }
    return out;
}

char *value(unsigned char v) {
    char *out;
    switch (v) {
    case 1:
        out = "A";
        break;
    case 2:
        out = "2";
        break;
    case 3:
        out = "3";
        break;
    case 4:
        out = "4";
        break;
    case 5:
        out = "5";
        break;
    case 6:
        out = "6";
        break;
    case 7:
        out = "7";
        break;
    case 8:
        out = "8";
        break;
    case 9:
        out = "9";
        break;
    case 10:
        out = "10";
        break;
    case 11:
        out = "J";
        break;
    case 12:
        out = "Q";
        break;
    case 13:
        out = "K";
        break;
    default:
        out = "_";
        break;
    }
    return out;
}

void print_seed(Seed s) {
    printf("[*] Seed: [");
    for (int i = 0; i < 32; i++) {
        if (i != 0) {
            printf(", ");
        }
        printf("%u", s[i]);
    }
    printf("]\n");
}

void print_card(uint8_t c) {
    if (c < 52) {
        int v = (c % 13) + 1;
        int s = c / 13;
        printf("%s%s", value(v), suit(s));
    } else {
        printf("__");
    }
}

void print_floor(Game *g, Status *s) {
    Pile piles[FLOOR_SIZE];
    memcpy(&piles, read_floor(&g), FLOOR_SIZE * sizeof(Pile));
    printf("Floor: ");
    for (int i = 0; i < FLOOR_SIZE; i++) {
        if (i != 0) {
            printf(", ");
        }
        printf("%c=", 'A' + i);
        Pile p = piles[i];
        bool single = p.cards[1] >= 52;
        char *wrap = "[]";
        if (p.build) {
            wrap = "{}";
        }
        char *owned = "";
        if (p.owner == s->turn) {
            owned = "*";
        }
        if (single) {
            printf("(");
        } else {
            printf("%s%s%c", owned, value(p.value), wrap[0]);
        }
        for (int j = 0; j < 20; j++) {
            if (p.cards[j] < 52) {
                if (j != 0) {
                    printf(" + ");
                }
                print_card(p.cards[j]);
            }
        }
        if (single) {
            printf(")");
        } else {
            printf("%c", wrap[1]);
        }
    }
    printf("\n");
}

void print_hand(Game *g) {
    uint8_t cards[HAND_SIZE];
    memcpy(&cards, read_hands(&g), HAND_SIZE * sizeof(uint8_t));
    printf("Hand: ");
    for (int i = 0; i < HAND_SIZE; i++) {
        if (i != 0) {
            printf(", ");
        }
        printf("%d=(", i + 1);
        print_card(cards[i]);
        printf(")");
    }
    printf("\n");
}

void print_scores(Scorecard opp, Scorecard dealer) {
    printf(
        "[*] Scores:\n\n"
        "Player | Aces | Most Cards | Most Spades | 10♦ | 2♠ | Suipis | Total\n"
        "------ | ---- | ---------- | ----------- | --- | -- | ------ | -----\n"
        "Opp    |    %d |          %d |           %d |   %d |  %d |      %d |  "
        "%d\n"
        "Dealer |    %d |          %d |           %d |   %d |  %d |      %d |  "
        "%d\n",
        opp.aces, opp.most_cards, opp.most_spades, opp.ten_of_diamonds,
        opp.two_of_spades, opp.suipi_count, opp.total, dealer.aces,
        dealer.most_cards, dealer.most_spades, dealer.ten_of_diamonds,
        dealer.two_of_spades, dealer.suipi_count, dealer.total);
}

char *get_move() {
    char *annotation = calloc(64, sizeof(char));
    printf("> Input your move below:\n");
    scanf("%63s", annotation);
    return annotation;
}

Seed *load_seed(char *seed_path) {
    uint8_t seed[32];
    FILE *fp;
    char line[8];
    struct stat status;
    if (stat(seed_path, &status) != 0) {
        return NULL;
    }
    fp = fopen(seed_path, "r");
    int i = 0;
    while (fgets(line, sizeof(line), fp) != NULL) {
        sscanf(line, "%hhu\n", &seed[i]);
        i++;
        if (i >= 32) {
            break;
        }
    }
    fclose(fp);
    Seed *ptr = malloc(sizeof(Seed));
    memcpy(ptr, seed, sizeof(Seed));
    return ptr;
}

int main(int argc, char *argv[]) {
    Seed *seed = NULL;
    if (argc > 1) {
        seed = load_seed(argv[1]);
    }
    Game *g = new_game(seed);
    Status *s = status(&g);
    uint8_t gameIndex = s->game;
    uint8_t roundIndex = s->round;
    print_seed(s->seed);
    while (s->game < 2) {
        if (s->turn) {
            printf("\n[*] Dealer's turn:\n");
        } else {
            printf("\n[*] Opponent's turn:\n");
        }
        print_floor(g, s);
        print_hand(g);
        char *error = NULL;
        do {
            if (error != NULL) {
                printf("[!] %s\n", error);
            }
            char *m = get_move();
            error = (char *)apply_move(&g, m);
            free(m);
        } while (strcmp(error, "") != 0);
        free(error);
        next_turn(&g);
        s = status(&g);
        if (s->floor == 0) {
            printf("\n\n ===== SUIPI! =====\n\n");
        }
        if (gameIndex != s->game) {
            Scorecard scores[4];
            memcpy(&scores, get_scores(&g), 4 * sizeof(Scorecard));
            print_scores(scores[(gameIndex * 2)], scores[(gameIndex * 2) + 1]);
            printf("\n\n ===== Next Game =====\n\n");
            gameIndex = s->game;
            roundIndex = s->round;
        } else if (roundIndex != s->round) {
            printf("\n\n ===== Next Round =====\n\n");
            roundIndex = s->round;
        }
    }
    printf("\n\n\n");
    free(seed);
    free(g);
    free(s);
    return 0;
}
