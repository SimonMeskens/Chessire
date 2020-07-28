/*

Add transposition tables too!
No need for quiescence search?

Source: https://www.chessprogramming.org/NegaScout

int NegaScout ( position p; int alpha, beta )
{                     /* compute minimax value of position p */
   int b, t, i;
   if ( d == maxdepth )
      return quiesce(p, alpha, beta);           /* leaf node */
   determine successors p_1,...,p_w of p;
   b = beta;
   for ( i = 1; i <= w; i++ ) {
      t = -NegaScout ( p_i, -b, -alpha );
      if ( (t > a) && (t < beta) && (i > 1) )
         t = -NegaScout ( p_i, -beta, -alpha ); /* re-search */
      alpha = max( alpha, t );
      if ( alpha >= beta )
         return alpha;                            /* cut-off */
      b = alpha + 1;                  /* set new null window */
   }
   return alpha;
}

*/
