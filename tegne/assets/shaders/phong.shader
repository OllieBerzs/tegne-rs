vert.spv                                                                                                                    00000010764 00000000000 0005351                                                                                                      ustar                                                                                                                                                                                                                                                          #     p                 GLSL.std.450                      main       .   1   5   9   L   Y   ^   `   f   i   k        �   
 GL_GOOGLE_cpp_style_line_directive    GL_GOOGLE_include_directive      main      	   modelspace_position      in_modelspace_position       worldspace_position      Constants            model_matrix            albedo_index            sampler_index        object       screenspace_position          Light             coords           color     %   WorldObject   %       world_matrix      %      lights    %      camera_position   %      time      %      light_matrices    %      cascade_splits    %      bias      '   world     .   out_modelspace_position   1   out_worldspace_position  	 5   out_screenspace_position      9   out_lightspace_position   L   out_normal    Y   in_normal     ^   out_uv    `   in_uv     d   gl_PerVertex      d       gl_Position   d      gl_PointSize      d      gl_ClipDistance   d      gl_CullDistance   f         i   out_color     k   in_color      m   MaterialObject    m       albedo_tint   m      font_width    m      font_border_tint      m      font_edge     m      font_border_offset    m      font_border_width     m      font_border_edge      m      arg_1     m      arg_2     m   	   arg_3     m   
   arg_4     o   material    G            H            H         #       H               H        #   @   H        #   D   G        H          #       H         #      G  #          G  $      @   H  %          H  %       #       H  %             H  %      #   @   H  %      #   �   H  %      #   �   H  %         H  %      #   �   H  %            H  %      #   �  H  %      #   �  G  %      G  '   "       G  '   !       G  .         G  1         G  5         G  9         G  L          G  Y         G  ^         G  `         H  d              H  d            H  d            H  d            G  d      G  i         G  k         H  m       #       H  m      #      H  m      #      H  m      #      H  m      #       H  m      #   (   H  m      #   ,   H  m      #   0   H  m      #   @   H  m   	   #   P   H  m   
   #   `   G  m      G  o   "      G  o   !            !                                          
                  
   ;           +          �?                                           	      ;        	   +                  	                    !           +  !   "        #       "     $      "    	 %      #   
      $            &      %   ;  &   '         (            -      
   ;  -   .      ;  -   1         4         ;  4   5        7      "      8      7   ;  8   9      +     :      +     @      +     F      ;  -   L        Q   
      ;     Y        \            ]      \   ;  ]   ^         _      \   ;  _   `      +  !   b        c      b     d         c   c      e      d   ;  e   f      ;  4   i         j         ;  j   k        m   
      
      \                        n      m   ;  n   o      6               �     ;     	      ;           ;           =  
         Q               Q              Q              P                    >  	      A              =           =        	   �              >        A  (   )   '      =     *   )   =     +      �     ,   *   +   >     ,   =     /   	   O  
   0   /   /             >  .   0   =     2      O  
   3   2   2             >  1   3   =     6      >  5   6   A  (   ;   '   :      =     <   ;   =     =      �     >   <   =   A  4   ?   9      >  ?   >   A  (   A   '   :   @   =     B   A   =     C      �     D   B   C   A  4   E   9   @   >  E   D   A  (   G   '   :   F   =     H   G   =     I      �     J   H   I   A  4   K   9   F   >  K   J   A     M         =     N   M        O      "   N   T     P   O   Q     R   P       O  
   S   R   R             Q     T   P      O  
   U   T   T             Q     V   P      O  
   W   V   V             P  Q   X   S   U   W   =  
   Z   Y   �  
   [   X   Z   >  L   [   =  \   a   `   >  ^   a   =     g      A  4   h   f      >  h   g   =     l   k   >  i   l   �  8              frag.spv                                                                                                                    00000031234 00000000000 0005303                                                                                                      ustar                                                                                                                                                                                                                                                          #     �             2        GLSL.std.450                     main    %   �   �   t  �  �  �  �               �   
 GL_GOOGLE_cpp_style_line_directive    GL_GOOGLE_include_directive      main         Light            coords          color        calc_dir_light(struct-Light-vf4-vf41;vf3;f1;         light        cam_dir      shadow       calc_point_light(struct-Light-vf4-vf41;vf3;vf3;f1;       light        cam_dir      pos      shadow       calc_shadow(struct-Light-vf4-vf41;       light        phong(    !   fragment(     #   normal    %   in_normal     (   light_dir     1   diff      7   reflect_dir   <   spec      C   ambient   J   diffuse   P   specular      a   normal    d   light_dir     k   diff      p   reflect_dir   u   spec      {   distance      �   attenuation   �   ambient   �   diffuse   �   specular      �   in_screenspace_position   �   Light     �       coords    �      color     �   WorldObject   �       world_matrix      �      lights    �      camera_position   �      time      �      light_matrices    �      cascade_splits    �      bias      �   world     �   shadow_index      �   normal    �   light_dir     �   cosTheta      �   bias      �   shadow_coord      �   in_lightspace_position    �   uv      depth       shadow      strength        texel_size      shadow_maps     samplers      *  x     5  y     @  off   p  cam_dir   t  in_modelspace_position    x  shadow    y  param     �  lighting      �  i     �  light     �  param     �  param     �  param     �  param     �  param     �  param     �  param     �  out_color     �  textures      �  Constants     �      model_matrix      �     albedo_index      �     sampler_index     �  object    �  in_uv     �  MaterialObject    �      albedo_tint   �     font_width    �     font_border_tint      �     font_edge     �     font_border_offset    �     font_border_width     �     font_border_edge      �     arg_1     �     arg_2     �  	   arg_3     �  
   arg_4     �  material      �  in_color      �  framebuffer   �  in_worldspace_position  G  %          G  �         H  �       #       H  �      #      G  �          G  �      @   H  �          H  �       #       H  �             H  �      #   @   H  �      #   �   H  �      #   �   H  �         H  �      #   �   H  �            H  �      #   �  H  �      #   �  G  �      G  �   "       G  �   !       G  �         G    "      G    !       G    "      G    !      G  t        G  �         G  �  "      G  �  !       H  �         H  �      #       H  �            H  �     #   @   H  �     #   D   G  �     G  �        H  �      #       H  �     #      H  �     #      H  �     #      H  �     #       H  �     #   (   H  �     #   ,   H  �     #   0   H  �     #   @   H  �  	   #   P   H  �  
   #   `   G  �     G  �  "      G  �  !       G  �        G  �  "      G  �  !       G  �             !                                          	           
                  
               !     
   	         !     
   	            !        	   !           $      
   ;  $   %        )          +  )   *          +         +     5       +     A      B+     D   ���=+  )   E      +     Q      ?+     �     �?+     �   �Q�=+     �   o=   �         ;  �   �        �           +  �   �         �           �           �         +  �   �        �   �   �     �   �   �    	 �   �   �   
      �            �      �   ;  �   �      +  )   �      +  �   �          �           �      �      )   +  �   �      +  )   �      +  )   �      +     �   
�#<  �      �      �      �   ;  �   �        �            �      �   +  �   �       	                                  �              ;                         +  �                           ;          +  )                    	 !                            "  !    %  )      +     e     @   q     
   ;  $   t        z     �   ,  
   �  5   5   5   +  )   �        �        ;  �  �     +  �   �  d     �    �     �      �  ;  �  �        �  �   )   )      �  	   �  ;  �  �  	      �  	   )     �       �     �   ;  �  �       �  
      
      �                        �     �  ;  �  �     ;  �   �     ;    �      ;  $   �     6               �     9     �  !   �  8  6  
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
   [   Y   Z   =     \      �  
   ]   [   \   �  
   ^   X   ]   �  ^   8  6  
             7  	      7        7        7        �     ;     a      ;     d      ;     k      ;     p      ;     u      ;     {      ;     �      ;     �      ;     �      ;     �      =  
   b   %     
   c      E   b   >  a   c   A  +   e      *   =     f   e   O  
   g   f   f             =  
   h      �  
   i   g   h     
   j      E   i   >  d   j   =  
   l   a   =  
   m   d   �     n   l   m        o      (   n   5   >  k   o   =  
   q   d     
   r   q   =  
   s   a     
   t      G   r   s   >  p   t   =  
   v      =  
   w   p   �     x   v   w        y      (   x   5        z         y   A   >  u   z   A  +   |      *   =     }   |   O  
   ~   }   }             =  
         �  
   �   ~           �      B   �   >  {   �   =     �   {   �     �   �   �   �     �   �   �   =     �   {   =     �   {   �     �   �   �   �     �   �   �   �     �   �   �   �     �   �   �   >  �   �   A  +   �      E   =     �   �   O  
   �   �   �             �  
   �   �   D   >  �   �   =     �   k   A  +   �      E   =     �   �   O  
   �   �   �             �  
   �   �   �   >  �   �   =     �   u   �     �   Q   �   A  +   �      E   =     �   �   O  
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
   �   �   �   =     �      �  
   �   �   �   �  
   �   �   �   �  �   8  6               7  	      �     ;  �   �      ;     �      ;     �      ;     �      ;     �      ;  +   �      ;  �   �      ;          ;          ;  �        ;  �        ;  �   *     ;  �   5     ;  �   @     A  �   �   �   �   =     �   �   A  �   �   �   �   �   =     �   �   �  �   �   �   �   �  �       �  �   �   �   �  �   >  �   *   �  �   �  �   A  �   �   �   �   =     �   �   A  �   �   �   �   �   =     �   �   �  �   �   �   �   �  �       �  �   �   �   �  �   >  �   E   �  �   �  �   >  �   �   �  �   �  �   �  �   �  �   =  
   �   %     
   �      E   �   >  �   �   A  +   �      *   =     �   �   O  
   �   �   �               
   �   �     
   �      E   �   >  �   �   =  
   �   �   =  
   �   �   �     �   �   �        �      +   �   5   �   >  �   �   A  �   �   �   �   =     �   �   =     �   �        �         �        �         �   �     �   �   �   >  �   �   =     �   �        �      +   �   5   �   >  �   �   =  )   �   �   A  �   �   �   �   =     �   �   >  �   �   =     �   �   O  �   �   �   �          A     �   �   �   =     �   �   P  �      �   �   �  �     �      �  �       Q   P  �     Q   Q   �  �         >  �     A       �   �   =         =       �   �     	      A     
  �   �   =       
  �       	    >      >    5   >    *   =  )     �   A          =        A          =         V  "  #       d  !  $  #  g  %  &  $  *   o  �   '  &  P  �   (  �   �   �  �   )  (  '  >    )  =  )   +    ~  )   ,  +  >  *  ,  �  -  �  -  �  /  0      �  1  �  1  =  )   2  *  =  )   3    �  �   4  2  3  �  4  .  /  �  .  =  )   6    ~  )   7  6  >  5  7  �  8  �  8  �  :  ;      �  <  �  <  =  )   =  5  =  )   >    �  �   ?  =  >  �  ?  9  :  �  9  =  )   A  *  o     B  A  =  )   C  5  o     D  C  P  �   E  B  D  =  �   F    �  �   G  E  F  >  @  G  =  )   H  �   A    I    H  =    J  I  A    K      =    L  K  V  "  M  J  L  =  �   N  �   =  �   O  @  �  �   P  N  O  =     Q    Q     R  P      Q     S  P     P  
   T  R  S  Q  Q     U  T     Y     V  M  T  U  =     W    �     X  W  V  >    X  �  ;  �  ;  =  )   Y  5  �  )   Z  Y  E   >  5  Z  �  8  �  :  �  0  �  0  =  )   [  *  �  )   \  [  E   >  *  \  �  -  �  /  =  )   ]    �  �   ^  ]  *   �  `      �  ^  _  `  �  _  =  )   a    �  )   b  a  �   �  )   c  b  E   o     d  c       f        d  e  =     g    �     h  g  f  >    h  �  `  �  `  =     i    �  �   j  i  �   �  l      �  j  k  l  �  k  >    5   �  l  �  l  =     m    �  m  8  6               �      ;     p     ;     x     ;  	   y     ;     �     ;  �   �     ;  	   �     ;  	   �     ;     �     ;     �     ;  	   �     ;     �     ;     �     ;     �     A  q  r  �   �   =  
   s  r  =  
   u  t  �  
   v  s  u    
   w     E   v  >  p  w  A  z  {  �   E   *   =  �   |  {  Q     }  |      A  +   ~  y  *   >  ~  }  Q       |     A  +   �  y  E   >  �    9     �     y  >  x  �  >  �  �  >  �  *   �  �  �  �  �  �  �      �  �  �  �  =  )   �  �  �  �   �  �  �  �  �  �  �  �  �  =  )   �  �  A  z  �  �   E   �  =  �   �  �  Q     �  �      A  +   �  �  *   >  �  �  Q     �  �     A  +   �  �  E   >  �  �  A     �  �  *   �   =     �  �  �  �   �  �  5   �  �      �  �  �  �  �  �  =     �  �  >  �  �  =  
   �  p  >  �  �  =     �  x  >  �  �  9  
   �     �  �  �  =  
   �  �  �  
   �  �  �  >  �  �  �  �  �  �  A     �  �  *   �   =     �  �  �  �   �  �  �   �  �      �  �  �  �  �  �  =     �  �  >  �  �  =  
   �  p  >  �  �  =  
   �  t  >  �  �  =     �  x  >  �  �  9  
   �     �  �  �  �  =  
   �  �  �  
   �  �  �  >  �  �  �  �  �  �  �  �  �  �  �  �  �  �  =  )   �  �  �  )   �  �  E   >  �  �  �  �  �  �  =  
   �  �  Q     �  �      Q     �  �     Q     �  �     P     �  �  �  �  �   �  �  8  6     !          �  "   A  �  �  �  E   =  )   �  �  A    �  �  �  =    �  �  A  �  �  �  �   =  )   �  �  A    �    �  =    �  �  V  �  �  �  �  =  �   �  �  W     �  �  �  A  q  �  �  *   =  
   �  �  Q     �  �      Q     �  �     Q     �  �     P     �  �  �  �  �   �     �  �  �  =     �  �  �     �  �  �  9     �     �     �  �  �  >  �  �  �  8                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      