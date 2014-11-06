#version 330

layout(location = 0) in vec2 vertexCoord;
out vec2 texCoord_;
uniform vec2 pos;
uniform vec2 size;
uniform vec2 frameBufferSize;

void main(){
	mat4 positionMatrix = mat4(
		vec4(1.0,0.0,0.0,pos.x),
		vec4(0.0,1.0,0.0,pos.y),
		vec4(0.0,0.0,1.0,0.0),
		vec4(0.0,0.0,0.0,1.0)
	);

	mat4 orthoProjectionMatrix = mat4(
		vec4( 2.0/frameBufferSize.x, 0.0                  , 0.0,-1.0),
		vec4( 0.0                  ,-2.0/frameBufferSize.y, 0.0, 1.0),
		vec4( 0.0                  , 0.0                  , 1.0, 0.0),
		vec4( 0.0                  , 0.0                  , 0.0, 1.0)
	);
	
	gl_Position = vec4(vertexCoord*size,0.0,1.0)*positionMatrix*orthoProjectionMatrix;
}
