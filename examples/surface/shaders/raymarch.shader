�      #     v                 GLSL.std.450                      main       .   1   5   9   R   _   b   d   h   j   p        �   
 GL_GOOGLE_cpp_style_line_directive    GL_GOOGLE_include_directive      main      	   modelspace_position      in_modelspace_position       worldspace_position      Constants            model_matrix            albedo_index            sampler_index        object       screenspace_position          Light             coords           color     %   WorldObject   %       world_matrix      %      lights    %      camera_position   %      time      %      light_matrices    %      cascade_splits    %      pcf   %      bias      '   world     .   out_modelspace_position   1   out_worldspace_position  	 5   out_screenspace_position      9   out_lightspace_position   R   out_normal    _   in_normal     b   out_color     d   in_color      h   out_uv    j   in_uv     n   gl_PerVertex      n       gl_Position   n      gl_PointSize      n      gl_ClipDistance   n      gl_CullDistance   p         s   MaterialObject    s       arg_1     s      arg_2     s      arg_3     s      arg_4     s      arg_5     s      arg_6     s      arg_7     s      arg_8     u   material    G            H            H         #       H               H        #   @   H        #   D   G        H          #       H         #      G  #          G  $      @   H  %          H  %       #       H  %             H  %      #   @   H  %      #   �   H  %      #   �   H  %         H  %      #   �   H  %            H  %      #   �  H  %      #   �  H  %      #   �  G  %      G  '   "       G  '   !       G  .         G  1         G  5         G  9         G  R          G  _         G  b         G  d         G  h         G  j         H  n              H  n            H  n            H  n            G  n      H  s       #       H  s      #      H  s      #       H  s      #   0   H  s      #   @   H  s      #   P   H  s      #   `   H  s      #   p   G  s      G  u   "      G  u   !            !                                          
                  
   ;           +          �?                                           	      ;        	   +                  	                    !           +  !   "        #       "     $      "    
 %      #   
      $               &      %   ;  &   '         (            -      
   ;  -   .      ;  -   1         4         ;  4   5        7      "      8      7   ;  8   9      +     :      +     @      +     F      +     L      ;  -   R        W   
      ;     _      ;  4   b         c         ;  c   d        f            g      f   ;  g   h         i      f   ;  i   j      +  !   l        m      l     n         m   m      o      n   ;  o   p       
 s                              t      s   ;  t   u      6               �     ;     	      ;           ;           =  
         Q               Q              Q              P                    >  	      A              =           =        	   �              >        A  (   )   '      =     *   )   =     +      �     ,   *   +   >     ,   =     /   	   O  
   0   /   /             >  .   0   =     2      O  
   3   2   2             >  1   3   =     6      >  5   6   A  (   ;   '   :      =     <   ;   =     =      �     >   <   =   A  4   ?   9      >  ?   >   A  (   A   '   :   @   =     B   A   =     C      �     D   B   C   A  4   E   9   @   >  E   D   A  (   G   '   :   F   =     H   G   =     I      �     J   H   I   A  4   K   9   F   >  K   J   A  (   M   '   :   L   =     N   M   =     O      �     P   N   O   A  4   Q   9   L   >  Q   P   A     S         =     T   S        U      "   T   T     V   U   Q     X   V       O  
   Y   X   X             Q     Z   V      O  
   [   Z   Z             Q     \   V      O  
   ]   \   \             P  W   ^   Y   [   ]   =  
   `   _   �  
   a   ^   `   >  R   a   =     e   d   >  b   e   =  f   k   j   >  h   k   =     q   	   A  4   r   p      >  r   q   �  8  �'      #     q                GLSL.std.450                     main      J  h  j  k  l  m  p               �   
 GL_GOOGLE_cpp_style_line_directive    GL_GOOGLE_include_directive      main         rotate(f1;       a        sphere(vf3;vf3;f1;       pos      c        r        box(vf3;vf3;         pos      size         get_dist_to_scene(vf3;       pos   !   ray_march(vf3;vf3;       ray_orig          ray_dir   %   get_normal(vf3;   $   pos   (   get_light(vf3;    '   pos   *   fragment(     ,   s     /   c     >   dist      L   dist      b   plane_dist    e   box_pos   n   Light     n       coords    n      color     r   WorldObject   r       world_matrix      r      lights    r      camera_position   r      time      r      light_matrices    r      cascade_splits    r      pcf   r      bias      t   world     w   param     �   box_dist      �   param     �   param     �   sphere_dist   �   param     �   param     �   param     �   morph_dist    �   dist      �   dist_orig     �   i     �   pos   �   dist_scene    �   param     �   dist      �   param     �   e     �   normal    �   param     �   param     �   param     �   light_pos     �   light_dir     �   normal    �   param     �   light     �   dist        param       param       uv      in_uv     )  ray_orig      ,  ray_dir   5  color     7  dist      8  param     :  param     =  pos   C  light     D  param     J  out_color     Q  MaterialObject    Q      arg_1     Q     arg_2     Q     arg_3     Q     arg_4     Q     arg_5     Q     arg_6     Q     arg_7     Q     arg_8     S  material      T  Constants     T      model_matrix      T     albedo_index      T     sampler_index     V  object    [  textures      `  samplers      c  skybox    f  shadow_maps   h  in_normal     j  in_color      k  in_modelspace_position    l  in_worldspace_position    m  in_screenspace_position   p  in_lightspace_position  H  n       #       H  n      #      G  p          G  q      @   H  r          H  r       #       H  r             H  r      #   @   H  r      #   �   H  r      #   �   H  r         H  r      #   �   H  r            H  r      #   �  H  r      #   �  H  r      #   �  G  r      G  t   "       G  t   !       G          G  J         H  Q      #       H  Q     #      H  Q     #       H  Q     #   0   H  Q     #   @   H  Q     #   P   H  Q     #   `   H  Q     #   p   G  Q     G  S  "      G  S  !       H  T         H  T      #       H  T            H  T     #   @   H  T     #   D   G  T     G  [  "      G  [  !       G  `  "      G  `  !      G  c  "      G  c  !      G  f  "      G  f  !       G  h         G  j        G  k        G  l        G  m        G  p             !                                          	         !  
   	                             !                 !              !           !  #         +     7     �?+     8         Q           +  Q   R       +  Q   U      +  Q   X      +     g      @+     h     �@,     i   8   g   h     l           m   l        n   l   l   +  Q   o        p   n   o     q   m   o    
 r   m   p         q   l            s      r   ;  s   t        u          +  u   v         x         ,     �   7   7   7   +     �   �̌?+     �      ?   �      u   +  u   �       +  u   �   d     �   +     �     �B+     �   o�:+  u   �         �         ,     �   �   8   +     �     �@+     �      A,     �   �   �   �   +       
�#<           ;         +     $    ��+     *    �@,     +  8   *  8   +     1  ���>,     6  8   8   8      I     l   ;  I  J      
 Q  l   l   l   l   l   l   l   l      R     Q  ;  R  S       T  m   u   u      U  	   T  ;  U  V  	    	 W                           +  Q   X  d     Y  W  X     Z      Y  ;  Z  [        \  +  Q   ]       ^  \  ]     _      ^  ;  _  `       	 a                              b      a  ;  b  c        d  W  o      e      d  ;  e  f         g        ;  g  h        i     l   ;  i  j     ;  g  k     ;  g  l     ;  i  m       n  l   o      o     n  ;  o  p     6               �     9     P  *   �  8  6  	          
   7        �     ;     ,      ;     /      =     -           .         -   >  ,   .   =     0           1         0   >  /   1   =     2   /   =     3   ,        4   3   =     5   ,   =     6   /   P     9   2   4   P     :   5   6   P  	   ;   9   :   �  ;   8  6               7        7        7        �     ;     >      =     ?      =     @      �     A   ?   @        B      B   A   =     C      �     D   B   C   >  >   D   =     E   >   �  E   8  6               7        7        �     ;     L      =     H           I         H   =     J      �     K   I   J   >     K   =     M      P     N   8   8   8        O      (   M   N        P      B   O   A     S      R   =     T   S   A     V      U   =     W   V   A     Y      X   =     Z   Y        [      (   W   Z        \      (   T   [        ]      %   \   8   �     ^   P   ]   >  L   ^   =     _   L   �  _   8  6               7        �     ;     b      ;     e      ;     w      ;     �      ;     �      ;     �      ;     �      ;     �      ;     �      ;     �      ;     �      ;     �      A     c      U   =     d   c   >  b   d   =     f      >  e   f   =     j   e   �     k   j   i   >  e   k   A  x   y   t   v   =     z   y   >  w   z   9  	   {      w   =     |   e   O     }   |   |          �     ~   }   {   =        e   O     �      ~            >  e   �   =     �   e   >  �   �   >  �   �   9     �      �   �   >  �   �   =     �      >  �   �   >  �   i   >  �   �   9     �      �   �   �   >  �   �   =     �   �   =     �   �   A  x   �   t   v   =     �   �        �         �   �     �   �   �   �     �   �   �        �      .   �   �   �   >  �   �   =     �   �   =     �   b        �      %   �   �   >  �   �   =     �   �   �  �   8  6     !          7        7         �  "   ;     �      ;  �   �      ;     �      ;     �      ;     �      >  �   8   >  �   �   �  �   �  �   �  �   �       �  �   �  �   =  u   �   �   �  �   �   �   �   �  �   �   �   �  �   =     �      =     �       =     �   �   �     �   �   �   �     �   �   �   >  �   �   =     �   �   >  �   �   9     �      �   >  �   �   =     �   �   =     �   �   �     �   �   �   >  �   �   =     �   �   �  �   �   �   �   =     �   �   �  �   �   �   �   �  �   �   �   �   �  �       �  �   �   �   �  �   �  �   �  �   �  �   �  �   =  u   �   �   �  u   �   �   �   >  �   �   �  �   �  �   =     �   �   �  �   8  6     %       #   7     $   �  &   ;     �      ;     �      ;  �   �      ;     �      ;     �      ;     �      ;     �      =     �   $   >  �   �   9     �      �   >  �   �   >  �   �   =     �   �   =     �   $   =     �   �   O     �   �   �             �     �   �   �   >  �   �   9     �      �   =     �   $   =     �   �   O     �   �   �             �     �   �   �   >  �   �   9     �      �   =     �   $   =     �   �   O     �   �   �             �     �   �   �   >  �   �   9     �      �   P     �   �   �   �   P     �   �   �   �   �     �   �   �   >  �   �   =     �   �        �      E   �   �  �   8  6     (          7     '   �  )   ;     �      ;     �      ;     �      ;     �      ;     �      ;     �      ;          ;          >  �   �   =     �   �   =     �   '   �     �   �   �        �      E   �   >  �   �   =     �   '   >  �   �   9     �   %   �   >  �   �   =     �   �   =     �   �   �     �   �   �   �     �   �   �   �     �   �   �        �      +   �   8   7   >  �   �   =        '   =       �   �         �   �         g   �            >      =       �   >      9       !       >  �     A     	  '   U   =     
  	  �  �     
    �        �        �    =       �   =       �   =       '   �                     B     �  �         �    �    �  �       )       �        �        �    =       �   �         �   >  �     �    �    =       �   �    8  6     *          �  +   ;  �        ;     )     ;     ,     ;     5     ;     7     ;     8     ;     :     ;     =     ;     C     ;     D     =          �     !     g   P     "  7   7   �     #  !  "  >    #  A     %    U   =     &  %  �     '  &  $  A     (    U   >  (  '  >  )  +  A     -    R   =     .  -  A     /    U   =     0  /  �     2  0  1  P     3  .  2  7        4     E   3  >  ,  4  >  5  6  =     9  )  >  8  9  =     ;  ,  >  :  ;  9     <  !   8  :  >  7  <  =     >  )  =     ?  ,  =     @  7  �     A  ?  @  �     B  >  A  >  =  B  =     E  =  >  D  E  9     F  (   D  >  C  F  =     G  C  P     H  G  G  G  >  5  H  =     K  5  Q     L  K      Q     M  K     Q     N  K     P  l   O  L  M  N  7   >  J  O  �  8         test_and_write       filled_triangles       back