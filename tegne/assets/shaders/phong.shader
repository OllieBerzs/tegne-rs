vert.spv                                                                                                                    00000007600 00000000000 0005344                                                                                                      ustar                                                                                                                                                                                                                                                          #     W                 GLSL.std.450                      main    	      $   &   )   +   -   .   5   :   G   Q        �   
 GL_GOOGLE_cpp_style_line_directive    GL_GOOGLE_include_directive      main      	   out_normal       Constants            model_matrix            albedo_index            sampler_index        object       in_normal     $   out_uv    &   in_uv     )   out_color     +   in_color      -   out_worldspace_position   .   in_worldspace_position    5   out_modelspace_position  	 :   out_screenspace_position      ;   Light     ;       coords    ;      color     ?   WorldObject   ?       world_matrix      ?      light_matrix      ?      lights    ?      camera_position   ?      time      ?      cascade_splits    ?      shadow_index      A   world     G   out_lightspace_position   O   gl_PerVertex      O       gl_Position   O      gl_PointSize      O      gl_ClipDistance   O      gl_CullDistance   Q         T   MaterialObject    T       albedo_tint   T      font_width    T      font_border_tint      T      font_edge     T      font_border_offset    T      font_border_width     T      font_border_edge      T      arg_1     T      arg_2     T   	   arg_3     T   
   arg_4     V   material    G  	          H            H         #       H               H        #   @   H        #   D   G        G           G  $         G  &         G  )         G  +         G  -         G  .          G  5         G  :         H  ;       #       H  ;      #      G  >          H  ?          H  ?       #       H  ?             H  ?         H  ?      #   @   H  ?            H  ?      #   �   H  ?      #      H  ?      #     H  ?      #     H  ?      #     G  ?      G  A   "       G  A   !       G  G         H  O              H  O            H  O            H  O            G  O      H  T       #       H  T      #      H  T      #      H  T      #      H  T      #       H  T      #   (   H  T      #   ,   H  T      #   0   H  T      #   @   H  T   	   #   P   H  T   
   #   `   G  T      G  V   "      G  V   !            !                                        ;     	        
              
                                      	      ;        	   +                  	                             ;             "            #      "   ;  #   $         %      "   ;  %   &         (      
   ;  (   )         *      
   ;  *   +      ;  (   -      ;     .      +     0     �?;  (   5      ;  (   :        ;   
   
     <           +  <   =        >   ;   =    	 ?         >                  @      ?   ;  @   A         B         ;  (   G      +     H      +  <   M        N      M     O   
      N   N      P      O   ;  P   Q        T               "         
   
   
   
      U      T   ;  U   V      6               �     A              =                      "      T           Q  
             O                        Q  
            O                        Q  
            O                        P                 =            �     !          >  	   !   =  "   '   &   >  $   '   =  
   ,   +   >  )   ,   =     /   .   Q     1   /       Q     2   /      Q     3   /      P  
   4   1   2   3   0   >  -   4   A     6         =     7   6   =  
   8   -   �  
   9   7   8   >  5   9   A  B   C   A      =     D   C   =  
   E   5   �  
   F   D   E   >  :   F   A  B   I   A   H   =     J   I   =  
   K   5   �  
   L   J   K   >  G   L   =  
   R   :   A  (   S   Q      >  S   R   �  8                                                                                                                                  frag.spv                                                                                                                    00000027434 00000000000 0005312                                                                                                      ustar                                                                                                                                                                                                                                                          #     �             2        GLSL.std.450                     main    %   �   R  �  �  �  �  �               �   
 GL_GOOGLE_cpp_style_line_directive    GL_GOOGLE_include_directive      main         Light            coords          color        calc_dir_light(struct-Light-vf4-vf41;vf3;f1;         light        cam_dir      shadow       calc_point_light(struct-Light-vf4-vf41;vf3;vf3;f1;       light        cam_dir      pos      shadow       calc_shadow(struct-Light-vf4-vf41;       light        phong(    !   fragment(     #   normal    %   in_normal     (   light_dir     1   diff      7   reflect_dir   <   spec      C   ambient   J   diffuse   P   specular      c   normal    f   light_dir     m   diff      r   reflect_dir   w   spec      }   distance      �   attenuation   �   ambient   �   diffuse   �   specular      �   shadow    �   normal    �   proj_coords   �   in_lightspace_position    �   uv    �   current_depth     �   light_dir     �   bias      �   strength      �   texel_size    �   textures      �   Light     �       coords    �      color     �   WorldObject   �       world_matrix      �      light_matrix      �      lights    �      camera_position   �      time      �      cascade_splits    �      shadow_index      �   world     �   samplers        x       y       pcf_depth     M  cam_dir   R  in_modelspace_position    W  shadow    X  param     a  lighting      c  i     l  light     y  param     {  param     }  param     �  param     �  param     �  param     �  param     �  out_color     �  Constants     �      model_matrix      �     albedo_index      �     sampler_index     �  object    �  in_uv     �  MaterialObject    �      albedo_tint   �     font_width    �     font_border_tint      �     font_edge     �     font_border_offset    �     font_border_width     �     font_border_edge      �     arg_1     �     arg_2     �  	   arg_3     �  
   arg_4     �  material      �  in_color      �  in_screenspace_position   �  in_worldspace_position  G  %          G  �         G  �   "      G  �   !       H  �       #       H  �      #      G  �          H  �          H  �       #       H  �             H  �         H  �      #   @   H  �            H  �      #   �   H  �      #      H  �      #     H  �      #     H  �      #     G  �      G  �   "       G  �   !       G  �   "      G  �   !      G  R        G  �         H  �         H  �      #       H  �            H  �     #   @   H  �     #   D   G  �     G  �        H  �      #       H  �     #      H  �     #      H  �     #      H  �     #       H  �     #   (   H  �     #   ,   H  �     #   0   H  �     #   @   H  �  	   #   P   H  �  
   #   `   G  �     G  �  "      G  �  !       G  �        G  �        G  �             !                                          	           
                  
               !     
   	         !     
   	            !        	   !           $      
   ;  $   %        )          +  )   *          +         +     5       +     A      B+     D   ���=+  )   E      +     Q      ?+     \     �?+     �   �Q�=+     �   o=   �         ;  �   �        �           +  �   �         �           �            �      �   +  �   �      +     �   o�:+     �   ��8   �      )    	 �                            +  �   �   d     �   �   �      �       �   ;  �   �         �           �         +  �   �        �   �   �    	 �   �   �   �   
      
   )      �      �   ;  �   �      +  )   �         �      )      �       �     �   +  �   �        �   �   �      �       �   ;  �   �       +  )   �         �       �     �   �       )          +  �   0      +     B     @+  )   N        O     
   ;  �   R        Y     �   ,  
   b  5   5   5   +  )   j        �        ;  �  �       �  �   )   )      �  	   �  ;  �  �  	      �  	   )      �     �   ;  �  �       �  
      
      �                        �     �  ;  �  �     ;  �   �     ;  �   �     ;  �   �     6               �     9     �  !   �  8  6  
             7  	      7        7        �     ;     #      ;     (      ;     1      ;     7      ;     <      ;     C      ;     J      ;     P      =  
   &   %     
   '      E   &   >  #   '   A  +   ,      *   =     -   ,   O  
   .   -   -               
   /   .     
   0      E   /   >  (   0   =  
   2   #   =  
   3   (   �     4   2   3        6      (   4   5   >  1   6   =  
   8   (     
   9   8   =  
   :   #     
   ;      G   9   :   >  7   ;   =  
   =      =  
   >   7   �     ?   =   >        @      (   ?   5        B         @   A   >  <   B   A  +   F      E   =     G   F   O  
   H   G   G             �  
   I   H   D   >  C   I   =     K   1   A  +   L      E   =     M   L   O  
   N   M   M             �  
   O   N   K   >  J   O   =     R   <   �     S   Q   R   A  +   T      E   =     U   T   O  
   V   U   U             �  
   W   V   S   >  P   W   =  
   X   C   =  
   Y   J   =  
   Z   P   �  
   [   Y   Z   =     ]      �     ^   \   ]   �  
   _   [   ^   �  
   `   X   _   �  `   8  6  
             7  	      7        7        7        �     ;     c      ;     f      ;     m      ;     r      ;     w      ;     }      ;     �      ;     �      ;     �      ;     �      =  
   d   %     
   e      E   d   >  c   e   A  +   g      *   =     h   g   O  
   i   h   h             =  
   j      �  
   k   i   j     
   l      E   k   >  f   l   =  
   n   c   =  
   o   f   �     p   n   o        q      (   p   5   >  m   q   =  
   s   f     
   t   s   =  
   u   c     
   v      G   t   u   >  r   v   =  
   x      =  
   y   r   �     z   x   y        {      (   z   5        |         {   A   >  w   |   A  +   ~      *   =        ~   O  
   �                   =  
   �      �  
   �   �   �        �      B   �   >  }   �   =     �   }   �     �   �   �   �     �   \   �   =     �   }   =     �   }   �     �   �   �   �     �   �   �   �     �   �   �   �     �   \   �   >  �   �   A  +   �      E   =     �   �   O  
   �   �   �             �  
   �   �   D   >  �   �   =     �   m   A  +   �      E   =     �   �   O  
   �   �   �             �  
   �   �   �   >  �   �   =     �   w   �     �   Q   �   A  +   �      E   =     �   �   O  
   �   �   �             �  
   �   �   �   >  �   �   =     �   �   =  
   �   �   �  
   �   �   �   >  �   �   =     �   �   =  
   �   �   �  
   �   �   �   >  �   �   =     �   �   =  
   �   �   �  
   �   �   �   >  �   �   =  
   �   �   =  
   �   �   =  
   �   �   �  
   �   �   �   =     �      �     �   \   �   �  
   �   �   �   �  
   �   �   �   �  �   8  6               7  	      �     ;     �      ;     �      ;     �      ;  �   �      ;     �      ;     �      ;     �      ;  �   �      ;  �   �      ;  �        ;  �        ;          >  �   5   =  
   �   %     
   �      E   �   >  �   �   =     �   �   O  
   �   �   �             A  �   �   �   �   =     �   �   P  
   �   �   �   �   �  
   �   �   �   >  �   �   =  
   �   �   O  �   �   �   �          �  �   �   �   Q   P  �   �   Q   Q   �  �   �   �   �   >  �   �   A     �   �   �   =     �   �   >  �   �   A  +   �      *   =     �   �   O  
   �   �   �               
   �   �     
   �      E   �   >  �   �   =  
   �   �   =  
   �   �   �     �   �   �   �     �   \   �   �     �   �   �        �      (   �   �   >  �   �   >  �   *   A  �   �   �   �   =  )   �   �   A  �   �   �   �   =  �   �   �   A  �   �   �   �   =  �   �   �   V  �      �   �   d  �        g        *   o  �       P  �     \   \   �  �         >  �     =  )     �   ~  )   	    >    	  �  
  �  
  �          �    �    =  )       =  )     �   �          �        �    =  )     �   ~  )       >      �    �    �          �    �    =  )       =  )     �   �          �        �    A  �     �   �   =  )        A  �   !  �      =  �   "  !  A  �   #  �   �   =  �   $  #  V  �   %  "  $  =  �   &  �   =  )   '    o     (  '  =  )   )    o     *  )  P  �   +  (  *  =  �   ,  �   �  �   -  +  ,  �  �   .  &  -  W     /  %  .  Q     1  /      >    1  =     2  �   =     3  �   �     4  2  3  =     5    �    6  4  5  �     7  6  \   5   =     8  �   �     9  8  7  >  �   9  �    �    =  )   :    �  )   ;  :  E   >    ;  �    �    �    �    =  )   <    �  )   =  <  E   >    =  �  
  �    =  )   >  �   �  )   ?  >  �   �  )   @  ?  E   o     A  @       C        A  B  =     D  �   �     E  D  C  >  �   E  =     F  �   �    G  F  \   �  I      �  G  H  I  �  H  >  �   5   �  I  �  I  =     J  �   �  J  8  6               �      ;     M     ;     W     ;  	   X     ;     a     ;  �   c     ;  	   l     ;  	   y     ;     {     ;     }     ;  	   �     ;     �     ;     �     ;     �     A  O  P  �   N  =  
   Q  P  =     S  R  O  
   T  S  S            �  
   U  Q  T    
   V     E   U  >  M  V  A  Y  Z  �   �   *   =  �   [  Z  Q     \  [      A  +   ]  X  *   >  ]  \  Q     ^  [     A  +   _  X  E   >  _  ^  9     `     X  >  W  `  >  a  b  >  c  *   �  d  �  d  �  f  g      �  h  �  h  =  )   i  c  �    k  i  j  �  k  e  f  �  e  =  )   m  c  A  Y  n  �   �   m  =  �   o  n  Q     p  o      A  +   q  l  *   >  q  p  Q     r  o     A  +   s  l  E   >  s  r  A     t  l  *   �   =     u  t  �    v  u  5   �  x      �  v  w  �  �  w  =     z  l  >  y  z  =  
   |  M  >  {  |  =     ~  W  >  }  ~  9  
        y  {  }  =  
   �  a  �  
   �  �    >  a  �  �  x  �  �  A     �  l  *   �   =     �  �  �    �  �  \   �  �      �  �  �  �  �  �  =     �  l  >  �  �  =  
   �  M  >  �  �  =     �  R  O  
   �  �  �            >  �  �  =     �  W  >  �  �  9  
   �     �  �  �  �  =  
   �  a  �  
   �  �  �  >  a  �  �  �  �  �  �  x  �  x  �  g  �  g  =  )   �  c  �  )   �  �  E   >  c  �  �  d  �  f  =  
   �  a  Q     �  �      Q     �  �     Q     �  �     P     �  �  �  �  \   �  �  8  6     !          �  "   A  �  �  �  E   =  )   �  �  A  �   �  �   �  =  �   �  �  A  �  �  �  �   =  )   �  �  A  �   �  �   �  =  �   �  �  V  �   �  �  �  =  �   �  �  W     �  �  �  A  O  �  �  *   =  
   �  �  Q     �  �      Q     �  �     Q     �  �     P     �  �  �  �  \   �     �  �  �  =     �  �  �     �  �  �  9     �     �     �  �  �  >  �  �  �  8                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      